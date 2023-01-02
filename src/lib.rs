mod utils;

use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_thread as thread;

use std::time::Duration;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[wasm_bindgen]
pub fn process() {
    console_log!("ENTRA A PROCESS");
    for _ in 0..2 {
        thread::spawn(|| {
            for i in 1..3 {
                console_log!(
                    "hi number {} from the spawned thread {:?}!",
                    i,
                    thread::current().id()
                );
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    for i in 1..3 {
        console_log!(
            "hi number {} from the main thread {:?}!",
            i,
            thread::current().id()
        );
    }
}
