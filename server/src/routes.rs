use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::{register, test}, AppState};
use crate::handlers::health_checker_handler;

pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/register", post(register))
        .route("/api/test", post(test))
        .with_state(app_state)
}