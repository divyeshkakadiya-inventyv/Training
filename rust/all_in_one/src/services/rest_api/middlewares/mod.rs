use axum::Router;
use tower::ServiceBuilder;

use self::cors::get_cors_middleware;
pub mod cors;

pub fn get_middlewares(route: Router) -> Router {
    route.layer(ServiceBuilder::new().layer(get_cors_middleware()))
}