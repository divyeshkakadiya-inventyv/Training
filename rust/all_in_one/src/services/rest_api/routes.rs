use axum::Router;
use crate::services::rest_api::health_check::get_status_routes;

use super::{customers::get_customers_routes, emp::get_emp_routes, student::get_student_routes};


pub fn get_routes() -> Router { 
    let app = Router::new()
    .merge(get_status_routes())
    .merge(get_student_routes())
    .merge(get_customers_routes())
    .merge(get_emp_routes());
    app
}