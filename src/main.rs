#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate geojson;
extern crate num_traits;
extern crate osmpbfreader;
extern crate petgraph;
extern crate rstar;
extern crate serde;

mod graph;
use graph::Graph;

fn main() {
    let input_filename = std::env::args_os().nth(1).unwrap();
    let output_filename = std::env::args_os().nth(2).unwrap();

    let graph = Graph::new(input_filename);
    // let graph = Graph::read(output_filename);
    graph.write(output_filename);

    let start = [37.606201171875, 55.65113939983104];
    let finish = [37.61537432670593, 55.627620411517114];

    let (score, route) = graph.route(&start, &finish);
    println!("score = {}", score);
    println!("{:?}", route);
}
