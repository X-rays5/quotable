use worker::*;
use serde_json::{Value};
use serde::Serialize;

use crate::data;

#[derive(Default, Serialize)]
struct Res {
    count: u64,
    results: Vec<Value>
}

pub async fn handle_request(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let json: Value = serde_json::from_str(data::TAGS).unwrap();

    let json =  match json["tags"].as_array() {
        Some(val) => val,
        None => {
            return Response::error("Invalid tags data", 500)
        }
    };
    let mut response: Res = Default::default();
    response.count = json.len() as u64;
    response.results = json.clone();
    match serde_json::to_string(&response) {
        Ok(val) => {
            Response::ok(val)
        },
        Err(_) => {
            Response::error("Internal server error", 500)
        }
    }
}