extern crate console_error_panic_hook;
use std::io::Cursor;
use tract_onnx::prelude::*;
//use tract_tensorflow::prelude::*;
use rand::*;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("WASM INICIADO");
}

#[wasm_bindgen(js_name = funcion)]
pub fn funcion() {
    log(&format!("funcion dice: {} y {}", "hola mundo", "adios"));
}

#[wasm_bindgen(js_name = add)]
pub fn add(x: i32, y: i32) {
    let sum = x + y;
    log(&format!("{} + {} = {}", x, y, sum));
}

#[wasm_bindgen]
pub fn infer(model_data: &[u8], image_data: &[u8], image_height: i32, image_width: i32) -> String {
    let res: (f32, u32) = infer_impl(
        model_data,
        image_data,
        image_height as usize,
        image_width as usize,
    )
    .unwrap();
    return serde_json::to_string(&res).unwrap();
}

fn infer_impl(
    model_data: &[u8],
    image_data: &[u8],
    image_height: usize,
    image_width: usize,
) -> TractResult<(f32, u32)> {
    let mut model_data_mut = Cursor::new(model_data);
    let model = tract_onnx::onnx()
        // load the model
        .model_for_read(&mut model_data_mut)?
        // specify input type and shape
        .with_input_fact(
            0, 
            InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 100, 100, 3))
        )?
        // optimize the model
        .into_optimized()?
        // make the model runnable and fix its inputs and outputs
        .into_runnable()?;

    log("plan");
    Ok((0.0, 0))
}
