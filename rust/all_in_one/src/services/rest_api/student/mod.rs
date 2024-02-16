use axum::{routing::post, Router};
pub mod model;
pub mod service;
use service::*;

pub fn get_student_routes() -> Router { 
    let routes = Router::new()
    .route("/student/add", post(create))
    .route("/student/get-all", post(get_all))
    .route("/student/update/:id", post(update))
    .route("/student/get/:id", post(get))
    .route("/student/delete/:id", post(delete));
    routes
    
}