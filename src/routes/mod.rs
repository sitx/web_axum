mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;

use axum::{
    routing::{get, post}, Router
};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use path_variables::hardcoded_path;
use query_params::query_params;

pub fn create_routes() -> Router<> {
    //Router::new().route("/", get(hello_world))
    Router::new()
        .route("/", get(hello_world))
        .route( "/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json",post(mirror_body_json))
        .route("/path-variable/:id", get(path_variables))
        .route("/path-variable/15", get(hardcoded_path))
        .route("/query-params", get(query_params))
}
