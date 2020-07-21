use crate::utils::{json_response, Params, ResponseData};
use web_sys::{Request, Response};

pub fn index(_request: Request, _params: Params) -> Response {
    let response_data = ResponseData {
        message: "Hello, World!".to_string(),
        ..Default::default()
    };
    json_response(response_data, 200)
}

pub fn hello_user(_request: Request, params: Params) -> Response {
    let hello_user = format!(
        "Hello, {}!",
        params.iter().find(|&x| x.0 == "username").unwrap().1
    );
    let response_data = ResponseData {
        message: hello_user,
        ..Default::default()
    };
    json_response(response_data, 200)
}
