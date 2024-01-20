use axum::{routing::get, Router};

use crate::controller::handler::hello;

pub fn app() -> Router {
    Router::new().route("/", get(hello))
}
