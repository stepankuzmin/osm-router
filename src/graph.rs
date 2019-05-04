use std::collections::HashMap;
use std::io::{Read, Write};

use osmpbfreader::{NodeId, OsmObj};
use petgraph;
use rstar::{Point, RTree};

#[allow(dead_code)]
fn predicate(object: &OsmObj) -> bool {
  let tags = object.tags();
  object.is_way()
    && (tags.contains("highway", "motorway")
      || tags.contains("highway", "motorway_link")
      || tags.contains("highway", "trunk")
      || tags.contains("highway", "trunk_link")
      || tags.contains("highway", "primary")
      || tags.contains("highway", "primary_link")
      || tags.contains("highway", "secondary")
      || tags.contains("highway", "secondary_link")
      || tags.contains("highway", "tertiary")
      || tags.contains("highway", "tertiary_link")
      || tags.contains("highway", "unclassified")
      || tags.contains("highway", "residential")
      || tags.contains("highway", "living_street"))
}

#[allow(dead_code)]
fn read_osmpbf(filename: std::ffi::OsString) -> osmpbfreader::OsmPbfReader<std::fs::File> {
  let path = std::path::Path::new(&filename);
  let file = std::fs::File::open(&path).unwrap();
  osmpbfreader::OsmPbfReader::new(file)
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Node {
  pub id: i64,
  pub lon: f64,
  pub lat: f64,
}

impl Point for Node {
  type Scalar = f64;
  const DIMENSIONS: usize = 2;

  fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
    Node {
      id: 0,
      lon: generator(0),
      lat: generator(1),
    }
  }

  fn nth(&self, index: usize) -> Self::Scalar {
    match index {
      0 => self.lon,
      1 => self.lat,
      _ => unreachable!(),
    }
  }

  fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
    match index {
      0 => &mut self.lon,
      1 => &mut self.lat,
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Graph {
  data: petgraph::Graph<Node, i32>,
  nodes: HashMap<i64, petgraph::prelude::NodeIndex>,
  index: rstar::RTree<Node>,
}

impl Graph {
  #[allow(dead_code)]
  pub fn new(filename: std::ffi::OsString) -> Graph {
    let mut pbf = read_osmpbf(filename);
    let objects = pbf.get_objs_and_deps(predicate).unwrap();

    let mut weights = HashMap::new();

    weights.insert("motorway", 1);
    weights.insert("motorway_link", 2);
    weights.insert("trunk", 3);
    weights.insert("trunk_link", 4);
    weights.insert("primary", 5);
    weights.insert("primary_link", 6);
    weights.insert("secondary", 7);
    weights.insert("secondary_link", 8);
    weights.insert("tertiary", 9);
    weights.insert("tertiary_link", 10);
    weights.insert("unclassified", 11);
    weights.insert("residential", 12);
    weights.insert("living_street", 13);

    let mut nodes = HashMap::new();
    let mut graph = petgraph::Graph::new();
    let mut tree = RTree::new();

    for (_id, object) in &objects {
      match object {
        OsmObj::Node(osm_node) => {
          let NodeId(node_id) = osm_node.id;

          let node = Node {
            id: node_id,
            lat: (osm_node.decimicro_lat as f64) * 1e-7,
            lon: (osm_node.decimicro_lon as f64) * 1e-7,
          };

          let node_index = graph.add_node(node);
          nodes.insert(node_id, node_index);
          tree.insert(node);
        }
        OsmObj::Way(osm_way) => {
          for osm_node_ids in osm_way.nodes.windows(2) {
            let NodeId(node1_id) = osm_node_ids[0];
            let NodeId(node2_id) = osm_node_ids[1];

            let node1_index = nodes.get(&node1_id).unwrap();
            let node2_index = nodes.get(&node2_id).unwrap();

            let highway = osm_way.tags.get("highway").unwrap().as_str();
            let weight = weights.get(highway).unwrap();

            graph.add_edge(*node1_index, *node2_index, *weight);
          }
        }
        OsmObj::Relation(_) => {}
      };
    }

    Graph {
      data: graph,
      nodes: nodes,
      index: tree,
    }
  }

  pub fn nearest_node(&self, lonlat: &[f64]) -> Option<&Node> {
    let node = Node {
      id: 0,
      lon: lonlat[0],
      lat: lonlat[1],
    };

    self.index.nearest_neighbor(&node)
  }

  pub fn node_index(&self, node: &Node) -> Option<&petgraph::prelude::NodeIndex> {
    self.nodes.get(&node.id)
  }

  pub fn route(&self, start: &[f64], finish: &[f64]) -> (i32, Vec<Vec<f64>>) {
    let start_node = self.nearest_node(start).unwrap();
    let finish_node = self.nearest_node(finish).unwrap();

    let start_index = self.node_index(&start_node).unwrap();
    let finish_index = self.node_index(&finish_node).unwrap();

    let (score, path) = petgraph::algo::astar(
      &self.data,
      *start_index,
      |finish| finish == *finish_index,
      |e| *e.weight(),
      |_| 0,
    )
    .unwrap();

    let mut route = vec![];
    let nodes = self.data.raw_nodes();
    for node_index in path {
      let node = nodes.get(node_index.index()).unwrap();
      let node_weight = &node.weight;
      route.push(vec![node_weight.lon, node_weight.lat]);
    }

    (score, route)
  }

  #[allow(dead_code)]
  pub fn write(&self, filename: std::ffi::OsString) -> () {
    let graph_bin = bincode::serialize(&self).unwrap();
    let mut buffer = std::fs::File::create(filename).unwrap();
    buffer.write(&graph_bin).unwrap();
  }

  #[allow(dead_code)]
  pub fn read(filename: std::ffi::OsString) -> Graph {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();
    bincode::deserialize(&buffer).unwrap()
  }
}
