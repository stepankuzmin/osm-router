extern crate bincode;
extern crate osmpbfreader;
extern crate petgraph;

use std::collections::BTreeMap;
use std::collections::HashMap;

// use bincode::serialize;
use osmpbfreader::{NodeId, OsmId, OsmObj, WayId};
use petgraph::Graph;

type OsmGraph = Graph<i64, i64>;
type OsmObjects = BTreeMap<OsmId, OsmObj>;

// get ways with tags[highway] == motorway
fn predicate(object: &OsmObj) -> bool {
    object.is_way() && object.tags().contains("highway", "motorway")
}

fn create_graph(objects: OsmObjects) -> OsmGraph {
    let mut nodes = HashMap::new();
    let mut graph = Graph::new();

    for (_id, object) in &objects {
        match object {
            OsmObj::Node(node) => {
                let NodeId(node_id) = node.id;
                let node_index = graph.add_node(node_id);
                nodes.insert(node_id, node_index);
            }
            OsmObj::Way(way) => {
                for node_ids in way.nodes.windows(2) {
                    let WayId(way_id) = way.id;

                    let NodeId(node1_id) = node_ids[0];
                    let NodeId(node2_id) = node_ids[1];

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

fn main() {
    let filename = std::env::args_os().nth(1).unwrap();
    let path = std::path::Path::new(&filename);
    let file = std::fs::File::open(&path).unwrap();

    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let objects = pbf.get_objs_and_deps(predicate).unwrap();

    let graph = create_graph(objects);

    // let graph_encoded: Vec<u8> = serialize(&graph).unwrap();
    println!("graph: {:?}", graph);
}
