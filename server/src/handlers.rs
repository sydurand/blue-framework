use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use std::sync::Arc;

use crate::{AppState, model::{Agent, Request}};
use crate::response::GenericResponse;

pub async fn register(State(data): State<Arc<AppState>>, Json(body): Json<Agent>,) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = GenericResponse {
        message: "registered".to_string(),
        status: "success".to_string(),
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn test(State(data): State<Arc<AppState>>, Json(body): Json<Request>,) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!(">> {}", body.key1);
    
    let json_response = GenericResponse {
        message: "registered".to_string(),
        status: "success".to_string(),
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}