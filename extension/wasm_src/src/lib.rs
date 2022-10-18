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

#[wasm_bindgen(js_name = count)]
pub fn count(image_data: &[u8]) {
    let total = image_data.len();
    log(&format!(
        "Hay {} imágenes",
        total
    ));
    let iter = image_data.iter();
    for img in iter{
        log(&format!("img: {}", img));
    }
}
