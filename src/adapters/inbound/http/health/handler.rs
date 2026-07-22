use axum::Json;

use super::response::HealthResponse;

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse::healthy())
}