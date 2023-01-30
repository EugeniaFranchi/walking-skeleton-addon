mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
use async_std::task;
use futures::join;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/bootstrap.js")]
extern "C" {
    fn predict(i: i32) -> JsValue;
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasmaddon!");
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

async fn hello() -> String {
    console_log!("start HELLO");
    task::sleep(Duration::from_secs(5)).await;
    console_log!("HELLO");
    String::from("Hello")
}

async fn world() -> String {
    console_log!("start WORLD");
    task::sleep(Duration::from_secs(1)).await;
    console_log!("WORLD");
    String::from("World")
}

#[wasm_bindgen]
pub async fn process(length: i32) -> String {
    // Llama a JS para la predicción
    // https://stackoverflow.com/questions/71217860/how-to-call-an-async-javascript-import-function-from-webassembly-rust-in-a-nod
    // let a = JsFuture::from(js_sys::Promise::from(predict(0)));
    // let b = JsFuture::from(js_sys::Promise::from(predict(1)));
    // let c = JsFuture::from(js_sys::Promise::from(predict(2)));
    // join!(a,b,c);

    let (h,w) = join!(hello(), world());
    h + w.as_str()
}
