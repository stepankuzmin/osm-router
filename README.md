# OSM Router

Some experiments with OpenStreetMap and Rust.

![screenshot](https://raw.githubusercontent.com/stepankuzmin/osm-router/master/screenshot.png)

## Usage

```shell
git clone https://github.com/stepankuzmin/osm-router.git
cd osm-router
wget moscow.osm.pbf
cargo run -- moscow.osm.pbf
open debug.html
```

## Plans

- [x] Reading OSM PBF
- [x] Graph building
- [x] Graph serealization/deserealization
- [x] Simple edge weight calculating
- [x] Spatial index for nodes lookup
- [x] Simple routing
- [x] GeoJSON input/output
- [x] Web server
- [x] Web UI
