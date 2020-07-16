extern crate cfg_if;
extern crate wasm_bindgen;

#[macro_use]
pub mod macros;
pub mod controllers;
mod routes;
pub mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use web_sys::{ExtendableEvent, Request, Response};

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
    routes::run(request)
}
