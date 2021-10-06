use worker::*;
use serde_json::{Value};

use crate::data;

async fn random_number(min: u64, max: u64) -> u64 {
    let mut req = match worker::Fetch::Url(format!("https://random.bleep.workers.dev/?min={0}&max={1}", min, max).parse().unwrap()).send().await {
        Ok(val) => val,
        Err(_) => Response::error("", 400).unwrap()
    };
    let body = match req.text().await {
        Ok(val) => val,
        Err(_) => "0".to_string()
    };

    match body.parse::<u64>() {
        Ok(val) => val,
        Err(_) => 0
    }
}

pub async fn handle_request(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let json: Value = serde_json::from_str(data::QUOTES).unwrap();
    let json =  match json["quotes"].as_array() {
        Some(val) => val,
        None => {
            return Response::error("Invalid quotes data", 500)
        }
    };

    let num = random_number(0, json.len() as u64).await;
    Response::ok(json.get(num as usize).unwrap().to_string())
}