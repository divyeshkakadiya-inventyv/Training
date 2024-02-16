use axum::{routing::post, Router};
pub mod model;
pub mod service;
use service::*;

pub fn get_emp_routes() -> Router { 
    let routes = Router::new()
    .route("/emp/add", post(create))
    .route("/emp/get-all", post(get_all))
    .route("/emp/update/:id", post(update))
    .route("/emp/get/:id", post(get))
    .route("/emp/delete/:id", post(delete));
    routes
}