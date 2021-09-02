extern crate cfg_if;
extern crate waserv;
extern crate wasm_bindgen;

use crate::controllers::home::{index, hello};
use cfg_if::cfg_if;
use waserv::{router::Router, Waserv};
use wasm_bindgen::prelude::*;
use web_sys::{ExtendableEvent, Request, Response};

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
    let mut router = Router::new();
    router.get("/", Box::new(index));
    router.get("/hello/:name", Box::new(hello));
    let mut ww = Waserv::new();
    ww.mount(router);
    ww.handle(request)
}
