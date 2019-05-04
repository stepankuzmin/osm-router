use std::io;
use std::path::Path;
use std::ffi::OsString;
use std::num::ParseFloatError;

pub fn build_graph_filename(input_filename: &OsString) -> io::Result<OsString>{
    Path::new(input_filename)
        .file_stem()
        .map(|stem| {
            let mut stem = stem.to_os_string();
            stem.push(".graph");
            stem
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "oh no!"))
}

pub fn parse_waypoints(waypoints: &str) -> Result<Vec<Vec<f64>>, ParseFloatError> {
    waypoints
        .split(';')
        .map(|waypoint| waypoint.split(',').map(|c| c.parse::<f64>()).collect())
        .collect()
}