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
    
    println!(">> Register");

    let _query_res = sqlx::query(r#"INSERT INTO Agent (id, implant, created_at, last_seen, os, ip, username, hostname) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
        .bind(body.id.to_string())
        .bind(body.implant)
        .bind(body.created_at)
        .bind(body.last_seen)
        .bind(body.os)
        .bind(body.ip)
        .bind(body.username)
        .bind(body.hostname)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());
    
    let json_response = GenericResponse {
        message: "agent registered".to_string(),
        status: "success".to_string(),
    };
    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn test(State(data): State<Arc<AppState>>, Path(param): Path<String>, Json(body): Json<Request>,) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!(">> {}", body.key1);
    println!(">> {}", param);

    let _query_res = sqlx::query(r#"INSERT INTO Request (key1, key2) VALUES (?, ?)"#)
        .bind(body.key1)
        .bind(body.key2)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    let json_response = GenericResponse {
        message: "test successful".to_string(),
        status: "success".to_string(),
    };

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}