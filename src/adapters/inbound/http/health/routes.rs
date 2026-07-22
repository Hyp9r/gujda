use axum::{
    Router,
    routing::get,
};

use super::handler::health_check;

pub fn router() -> Router {
    Router::new().route("/health", get(health_check))
}