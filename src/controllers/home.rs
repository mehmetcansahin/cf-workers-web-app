use serde::Serialize;
use waserv::{response::json, Params};
use web_sys::{Request, Response};

#[derive(Serialize, Debug)]
struct Data {
    message: String,
}

pub fn index(_request: Request, _params: Params) -> Response {
    let data = Data {
        message: "Hello, World!".to_string(),
    };
    let body = serde_json::to_string(&data).unwrap();
    json(body)
}

pub fn hello(_request: Request, params: Params) -> Response {
    let name = &params.iter().find(|param| param.0.eq("name")).unwrap().1;
    let hello = format!("Hello, {}!", name);
    let data = Data { message: hello };
    let body = serde_json::to_string(&data).unwrap();
    json(body)
}
