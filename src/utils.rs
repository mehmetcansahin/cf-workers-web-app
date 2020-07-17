use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use web_sys::{Response, ResponseInit};

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub type Params = Vec<(String, String)>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseData {
    pub code: u16,
    pub status: String,
    pub message: String,
}

impl Default for ResponseData {
    fn default() -> Self {
        ResponseData {
            code: 200,
            status: "success".to_string(),
            message: "".to_string(),
        }
    }
}

// TODO: macro, response_data type <T>
pub fn json_response(response_data: <T>) -> Response {
    let json = serde_json::to_string(&response_data).unwrap();
    Response::new_with_opt_str_and_init(
        Some(&json),
        ResponseInit::new()
            .headers(
                headers! {
                    "Content-Type" => "application/json",
                    "Cache-Control" => "no-cache"
                }
                .as_ref(),
            )
            .status(response_data.code)
            .as_ref(),
    )
    .unwrap()
}
