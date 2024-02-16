use axum::{routing::post, Router};
pub mod model;
pub mod service;
use service::*;

pub fn get_customers_routes() -> Router { 
    let routes = Router::new()
    .route("/customer/add", post(create))
    .route("/customer/get-all", post(get_all))
    .route("/customer/update/:id", post(update))
    .route("/customer/get/:id", post(get))
    .route("/customer/delete/:id", post(delete));
    routes
    
}