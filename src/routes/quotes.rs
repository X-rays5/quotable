use worker::*;
use serde_json::{Value};
use serde::Serialize;

use crate::data;

#[derive(Default, Serialize)]
struct Res {
    count: u64,
    results: Vec<Value>
}

pub async fn handle_request_specific(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = match ctx.param("id") {
        Some(val) => val,
        None => {
            return Response::error("Id missing", 400)
        }
    };
    let json: Value = serde_json::from_str(data::QUOTES).unwrap();
    let json =  match json["quotes"].as_array() {
        Some(val) => val,
        None => {
            return Response::error("Invalid quotes data", 500)
        }
    };

    for val in json {
        if val["_id"].as_str().unwrap() == id.as_str() {
            return Response::ok(val.to_string());
        }
    }

    Response::error("Quote does not exist", 404)
}

pub async fn handle_request(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let json: Value = serde_json::from_str(data::QUOTES).unwrap();

    let json =  match json["quotes"].as_array() {
        Some(val) => val,
        None => {
            return Response::error("Invalid quotes data", 500)
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