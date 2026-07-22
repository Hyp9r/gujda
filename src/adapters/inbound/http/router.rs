use axum::Router;

use super::health;

pub fn create_router() -> Router {
    Router::new().merge(health::router())
}