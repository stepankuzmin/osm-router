#[macro_use]
extern crate actix_web;
extern crate bincode;
extern crate env_logger;
extern crate geojson;
extern crate log;
extern crate osmpbfreader;
extern crate petgraph;
extern crate rstar;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::{env, io};
use std::path::Path;

mod graph;
mod utils;
mod server;

use graph::Graph;

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let input_filename = std::env::args_os()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "oh no!"))?;

    let output_filename = utils::build_graph_filename(&input_filename)?;

    let graph = if Path::new(&output_filename).exists() {
        println!("Using graph {:?}", output_filename);
        let graph = Graph::read(output_filename);
        graph
    } else {
        println!("Building graph using {:?}", input_filename);
        let graph = Graph::new(input_filename);
        println!("Writing graph to {:?}", output_filename);
        graph.write(output_filename);
        graph
    };

    server::start(graph)
}
