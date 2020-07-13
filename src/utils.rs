use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

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

#[macro_export]
macro_rules! headers(
    { $($key:expr => $value:expr),+ } => {
        {
            let headers = ::web_sys::Headers::new().unwrap();
            $(
                headers.set($key, $value).unwrap();
            )+
            headers
        }
     };
     () => { ::web_sys::Headers::new().unwrap() };
);

pub type Params = Vec<(String, String)>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseData {
    pub status: String,
    pub message: String,
}

impl Default for ResponseData {
    fn default() -> Self {
        ResponseData {
            status: "success".to_string(),
            message: "".to_string(),
        }
    }
}
