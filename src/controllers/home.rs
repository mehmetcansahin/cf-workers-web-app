use serde::Serialize;
use web_sys::{Request, Response};
use webworker::{response::response, Params};

#[derive(Serialize, Debug)]
struct Data {
    message: String,
}

pub fn index(_request: Request, _params: Params) -> Response {
    let data = Data {
        message: "Hello, World!".to_string(),
    };
    let body = serde_json::to_string(&data).unwrap();
    let headers = headers! {
        "Content-Type" => "application/json",
        "Cache-Control" => "no-cache"
    };
    response(body, headers, Some(200))
}
