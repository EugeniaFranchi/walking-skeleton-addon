use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    log("WASM INICIADO");
}

#[wasm_bindgen(js_name = funcion)]
pub fn funcion() {
    log(&format!(
        "funcion dice: {} y {}",
        "hola mundo",
        "adios"
    ));
}

#[wasm_bindgen(js_name = add)]
pub fn add(x: i32, y: i32) {
    let sum = x + y;
    log(&format!(
        "{} + {} = {}",
        x,
        y,
        sum
    ));
}