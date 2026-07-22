use axum::Json;
use tracing::debug;

use super::response::HealthResponse;

pub async fn health_check() -> Json<HealthResponse> {
    debug!("health check requested");
    
    Json(HealthResponse::healthy())
}