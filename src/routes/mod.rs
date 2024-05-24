mod hello_world;
use axum::{
    routing::{get, post}, Router
};
use hello_world::hello_world;

pub fn create_routes() -> Router<> {
    //Router::new().route("/", get(hello_world))
    Router::new().route("/", get(hello_world))
}

