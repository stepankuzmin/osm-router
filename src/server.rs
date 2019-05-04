use std::io;
use std::sync::{Arc};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, error, web};
use actix_web::http::StatusCode;
use geojson::{Geometry, Value};
use serde_json;

use super::graph::Graph;
use super::utils::parse_waypoints;

#[derive(Debug)]
struct State {
    graph: Arc<Graph>
}

#[get("/{waypoints}")]
fn route(state: web::Data<State>, req: HttpRequest, path: web::Path<(String,)>) -> Result<HttpResponse> {
    let waypoints = parse_waypoints(&path.0)
        .map_err(|_| error::ErrorBadRequest("bad request"))?;

    let (_, route) = state.graph.route(&waypoints[0], &waypoints[1]);

    let geometry = Geometry::new(Value::LineString(route));
    let geometry = serde_json::to_string(&geometry)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(geometry))
}

pub fn start(graph: Graph) -> io::Result<()> {
    let sys = actix_rt::System::new("server");
    let graph = Arc::new(graph);

    HttpServer::new(move || {
        App::new()
            .data(State { graph: graph.clone() })
            .service(route)
        })
        .bind("127.0.0.1:8080")?
        .start();

    println!("Starting http server at 127.0.0.1:8080");
    sys.run()
}