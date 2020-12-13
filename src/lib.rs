extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate webworker;

use crate::controllers::home::index;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use web_sys::{ExtendableEvent, Request, Response};
use webworker::{router::Router, WebWorker};

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
pub fn handle_event(_event: ExtendableEvent, request: Request) -> Response {
    let mut ww = WebWorker::new();
    let mut router = Router::new();
    router.get("/", Box::new(index));
    ww.mount(router);
    ww.handle(request)
}
