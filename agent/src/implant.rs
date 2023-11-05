use std::{thread::sleep, time::Duration};
use serde::{Serialize, Deserialize};

use crate::Agent;

#[derive(Serialize, Deserialize)]
struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub fn run(api_agent: &ureq::Agent, agent: Agent) -> Result<(), ureq::Error> {
    let beacon_sleep = Duration::from_secs(5);

    let serialized = serde_json::to_string(&agent).unwrap();
    println!("▶️ Agent: {}", serialized);

    loop {
        println!("📡 Request sent!");
        //let res:String = api_agent.get(format!("http://127.0.0.1:3030/api/hello/{}", agent.id).as_str())
        match  api_agent.post("http://127.0.0.1:3030/api/register")
                    .set("Content-Type", "application/json")
                    .send_json(ureq::json!(&agent))
        {
            Ok(res) => {
                println!("✅ Registering is successful: {} - {}", res.status(), res.status_text());
                println!("▶️ {}", res.into_string()?);
            }

            Err(err) => {
                println!("❌ Error registering: {}", err);
            }
        };

        sleep(beacon_sleep);
    }
}