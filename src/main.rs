#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate osmpbfreader;
extern crate petgraph;
extern crate rstar;
extern crate serde;

mod graph;
// mod index;

use osmpbfreader::OsmObj;

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

fn read_osmpbf(filename: std::ffi::OsString) -> osmpbfreader::OsmPbfReader<std::fs::File> {
    let path = std::path::Path::new(&filename);
    let file = std::fs::File::open(&path).unwrap();
    osmpbfreader::OsmPbfReader::new(file)
}

fn main() {
    let input_filename = std::env::args_os().nth(1).unwrap();
    // let output_filename = std::env::args_os().nth(2).unwrap();

    let mut pbf = read_osmpbf(input_filename);
    let objects = pbf.get_objs_and_deps(predicate).unwrap();

    let graph = graph::create_graph(objects);

    let a = petgraph::prelude::NodeIndex::new(42699);
    let b = petgraph::prelude::NodeIndex::new(23327);

    let (score, path) =
        petgraph::algo::astar(&graph, a, |finish| finish == b, |e| *e.weight(), |_| 0).unwrap();

    println!("{:?}", path);
    println!("score = {:?}", score);

    let nodes = graph.raw_nodes();
    for node_index in path {
        let node = nodes.get(node_index.index()).unwrap();
        let node_weight = &node.weight;
        println!("[{:?}, {:?}],", node_weight.lon, node_weight.lat);
    }

    // let nodes = path
    //     .iter()
    //     .map(|node_index| graph.raw_nodes().get(node_index.index()))
    //     .collect();

    // let scores = petgraph::algo::dijkstra(&graph, a, Some(b), |e| *e.weight());
    // let scores: Vec<_> = scores.into_iter().map(|(n, s)| (&graph[n], s)).collect();
    // for (node, s) in scores {
    //     println!("{:?} {:?}", node, s);
    // }

    // let nodes = graph.raw_nodes();
    // for node_id in scores.keys() {
    //     let node = nodes.get(node_id.index()).unwrap();
    //     let node_weight = &node.weight;
    //     println!("[{:?}, {:?}],", node_weight.lon, node_weight.lat);
    // }

    // graph::write(graph, output_filename);

    // let graph = graph::read(output_filename);
    // println!("{:?}", graph);
}
