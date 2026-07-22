use axum::Router;
use tower_http::trace::TraceLayer;

use super::health;

pub fn create_router() -> Router {
    Router::new()
        .merge(health::router())
        .layer(TraceLayer::new_for_http())
}