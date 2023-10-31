use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;


#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct Request {
    pub key1: i32,
    pub key2: i32,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct Agent {
    pub id: String,
    pub implant: bool,
    pub created_at:DateTime<Utc>, 
    pub last_seen: Option<DateTime<Utc>>,
    pub os: String,
    pub ip: String,
    pub username: String,
    pub hostname: String,
}
