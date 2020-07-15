use crate::utils::{Params, ResponseData};
use web_sys::Request;

pub fn index(_request: Request, _params: Params) -> ResponseData {
    ResponseData {
        message: "Hello, World!".to_string(),
        ..Default::default()
    }
}

pub fn hello_user(_request: Request, params: Params) -> ResponseData {
    let hello_user = format!(
        "Hello, {}!",
        params.iter().find(|&x| x.0 == "username").unwrap().1
    );
    ResponseData {
        message: hello_user,
        ..Default::default()
    }
}
