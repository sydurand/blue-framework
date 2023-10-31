use std::{thread::sleep, time::Duration};

pub fn run(api_agent: &ureq::Agent, id: String) -> Result<(), ureq::Error> {
    let beacon_sleep = Duration::from_secs(5);

    loop {
        let res:String = api_agent.get(format!("http://127.0.0.1:3030/api/hello/{}", id).as_str())
        .call().expect("Err")
        .into_string().expect("Err");

        println!(">> {:?}", res);

        sleep(beacon_sleep);
    }
}