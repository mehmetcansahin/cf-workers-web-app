extern crate cfg_if;
extern crate wasm_bindgen;

use crate::controllers::home::index;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use web_sys::{ExtendableEvent, Request, Response};
use webworker::WebWorker;

mod controllers;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn handle_event(event: ExtendableEvent, request: Request) -> Response {
    let mut ww = WebWorker::default();
    ww.route("/GET/".to_string(), Box::new(index));
    ww.handle(event, request)
}
