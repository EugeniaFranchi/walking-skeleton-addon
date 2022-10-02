use railroad::RailroadNode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    // Set panic hook to get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    std::panic::set_hook(Box::new(console_error_panic_hook::hook))
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}