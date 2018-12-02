use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::{Read, Write};

use osmpbfreader::{NodeId, OsmId, OsmObj};
use petgraph;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
  pub id: i64,
  pub lat: f64,
  pub lon: f64,
}

pub type Graph = petgraph::Graph<Node, i32>;

pub fn create_graph(objects: BTreeMap<OsmId, OsmObj>) -> Graph {
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
  let mut graph = Graph::new();

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

  graph
}

pub fn write(graph: Graph, filename: std::ffi::OsString) -> () {
  let graph_bin = bincode::serialize(&graph).unwrap();
  let mut buffer = std::fs::File::create(filename).unwrap();
  buffer.write(&graph_bin).unwrap();
}

pub fn read(filename: std::ffi::OsString) -> Graph {
  let mut file = std::fs::File::open(filename).unwrap();
  let mut buffer = vec![];
  file.read_to_end(&mut buffer).unwrap();
  bincode::deserialize(&buffer).unwrap()
}
