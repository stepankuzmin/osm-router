#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate osmpbfreader;
extern crate petgraph;
extern crate serde;

mod graph;

use osmpbfreader::OsmObj;

fn predicate(object: &OsmObj) -> bool {
    object.is_way()
        && (object.tags().contains("highway", "motorway")
            || object.tags().contains("highway", "motorway_link")
            || object.tags().contains("highway", "trunk")
            || object.tags().contains("highway", "trunk_link")
            || object.tags().contains("highway", "primary")
            || object.tags().contains("highway", "primary_link")
            || object.tags().contains("highway", "secondary")
            || object.tags().contains("highway", "secondary_link")
            || object.tags().contains("highway", "tertiary")
            || object.tags().contains("highway", "tertiary_link")
            || object.tags().contains("highway", "unclassified")
            || object.tags().contains("highway", "residential")
            || object.tags().contains("highway", "living_street"))
}

fn read_osmpbf(filename: std::ffi::OsString) -> osmpbfreader::OsmPbfReader<std::fs::File> {
    let path = std::path::Path::new(&filename);
    let file = std::fs::File::open(&path).unwrap();
    osmpbfreader::OsmPbfReader::new(file)
}

fn main() {
    let input_filename = std::env::args_os().nth(1).unwrap();
    let output_filename = std::env::args_os().nth(2).unwrap();

    let mut pbf = read_osmpbf(input_filename);
    let objects = pbf.get_objs_and_deps(predicate).unwrap();

    let graph = graph::create_graph(objects);
    println!("{:?}", graph);

    graph::write(graph, output_filename);

    // let graph = graph::read(output_filename);
    // println!("{:?}", graph);
}
