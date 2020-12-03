use serde_json::json;
use web_sys::{Request, Response};
use webworker::{response_json, Params};

pub fn index(_request: Request, _params: Params) -> Response {
    let data = json!({
        "message": "Hello, World!".to_string()
    });
    response_json(data, None)
}
