extern crate cfg_if;
extern crate wasm_bindgen;

pub mod utils;

use cfg_if::cfg_if;
use path_tree::PathTree;
use utils::{Params, ResponseData};
use wasm_bindgen::prelude::*;
use web_sys::{ExtendableEvent, Request, Response, ResponseInit, Url};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub fn index(_request: Request, _params: Params) -> ResponseData {
    ResponseData {
        message: "Hello, World!".to_string(),
        ..Default::default()
    }
}

pub fn hello(_request: Request, params: Params) -> ResponseData {
    let hello_user = format!(
        "Hello, {}!",
        params.iter().find(|&x| x.0 == "username").unwrap().1
    );
    ResponseData {
        message: hello_user,
        ..Default::default()
    }
}

#[wasm_bindgen]
pub fn handle_event(_event: ExtendableEvent, request: Request) -> Response {
    let mut tree = PathTree::<Box<dyn Fn(Request, Params) -> ResponseData>>::new();
    tree.insert("/GET/", Box::new(index));
    tree.insert("/GET/hello/:username", Box::new(hello));

    let url = Url::new(&request.url()).unwrap();
    let path = "/".to_owned() + request.method().as_str() + &url.pathname();
    match tree.find(&path) {
        Some((node, params)) => {
            let params = params
                .iter()
                .map(|p| (p.0.to_string(), p.1.to_string()))
                .collect::<Params>();
            let response_data = node(request, params);
            let json = serde_json::to_string(&response_data).unwrap();
            return Response::new_with_opt_str_and_init(
                Some(&json),
                ResponseInit::new()
                    .headers(
                        headers! {
                            "Content-Type" => "application/json",
                            "Cache-Control" => "no-cache"
                        }
                        .as_ref(),
                    )
                    .status(200)
                    .as_ref(),
            )
            .unwrap();
        }
        None => {
            let error_data = ResponseData {
                status: "not_found".to_string(),
                message: "404 NOT FOUND".to_string(),
                ..Default::default()
            };
            let json = serde_json::to_string(&error_data).unwrap();
            return Response::new_with_opt_str_and_init(
                Some(&json),
                ResponseInit::new()
                    .headers(
                        headers! {
                            "Content-Type" => "application/json",
                            "Cache-Control" => "no-cache"
                        }
                        .as_ref(),
                    )
                    .status(404)
                    .as_ref(),
            )
            .unwrap();
        }
    }
}
