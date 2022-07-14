use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (mbutils::log(&format_args!($($t)*).to_string()))
}

//pub(crate) use console_log;
//pub use console_log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
