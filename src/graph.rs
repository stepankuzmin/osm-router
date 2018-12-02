use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::{Read, Write};

use osmpbfreader::{NodeId, OsmId, OsmObj, WayId};
use petgraph;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
  pub id: i64,
  pub decimicro_lat: i32,
  pub decimicro_lon: i32,
}

pub type Graph = petgraph::Graph<Node, i64>;

pub fn create_graph(objects: BTreeMap<OsmId, OsmObj>) -> Graph {
  let mut nodes = HashMap::new();
  let mut graph = Graph::new();

  for (_id, object) in &objects {
    match object {
      OsmObj::Node(osm_node) => {
        let NodeId(node_id) = osm_node.id;

        let node = Node {
          id: node_id,
          decimicro_lat: osm_node.decimicro_lat,
          decimicro_lon: osm_node.decimicro_lon,
        };

        let node_index = graph.add_node(node);
        nodes.insert(node_id, node_index);
      }
      OsmObj::Way(osm_way) => {
        let WayId(way_id) = osm_way.id;
        for osm_node_ids in osm_way.nodes.windows(2) {
          let NodeId(node1_id) = osm_node_ids[0];
          let NodeId(node2_id) = osm_node_ids[1];

          let node1_index = nodes.get(&node1_id).unwrap();
          let node2_index = nodes.get(&node2_id).unwrap();

          graph.add_edge(*node1_index, *node2_index, way_id);
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
