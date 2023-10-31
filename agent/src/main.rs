use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;
use std::time::Duration;

mod implant;

#[derive(Serialize, Deserialize)]
struct Agent {
    pub id: String,
    pub created_at:DateTime<Utc>, 
    pub last_seen: DateTime<Utc>,
}

fn main() {
    log::debug!("Agent launched!");
    let agent = Agent{id: Uuid::new_v4().to_string(), created_at: chrono::offset::Utc::now(), last_seen: chrono::offset::Utc::now()};

    let api_agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("ch_10_agent/0.1")
        .build();


    let serialized = serde_json::to_string(&agent).unwrap();

    println!("Hello, world {}", serialized);

    let _ = implant::run(&api_agent, agent.id);
}
