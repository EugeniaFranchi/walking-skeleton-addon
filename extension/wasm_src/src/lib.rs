extern crate console_error_panic_hook;
use std::io::Cursor;
use tract_onnx::prelude::*;
use wasm_bindgen::prelude::*;
use std::panic;


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

#[wasm_bindgen(js_name = infer)]
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
    // load the model

    log("CREATING MODEL");
    let mut model_data_mut = Cursor::new(model_data);
    let r = tract_onnx::onnx().model_for_read(&mut model_data_mut);
    let mut model = r.unwrap();

    // specify input type and shape
    log("SPECIFY INPUT TYPE AND SHAPE");
    model.set_input_fact(
        0,
        f32::fact(&[200, 200, 1]).into(),
    )?;
    // optimize the model and get an execution plan
    log("optimize the model and get an execution plan");
    log("into_optimized");
    let model = model.into_optimized()?;
    log("simpleplan::new()");
    let plan = SimplePlan::new(&model)?;

    // open image, resize it and make a Tensor out of it
    log("open image, resize it and make a Tensor out of it");
    let image = image::load_from_memory(image_data).unwrap().to_rgb8();
    let resized = image::imageops::resize(
        &image,
        image_height as u32,
        image_width as u32,
        ::image::imageops::FilterType::Triangle,
    );
    let image: Tensor =
        tract_ndarray::Array4::from_shape_fn((1, image_height, image_width, 1), |(_, y, x, c)| {
            resized[(x as _, y as _)][c] as f32 / 255.0
        })
        .into();

    // run the plan on the input
    log("run the plan on the input");
    let result = plan.run(tvec!(image))?;

    // find and display the max value with its index
    log("find and display the max value with its index");
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    match best {
        Some(t) => Ok(t),
        None => Ok((0.0, 0)),
    }
}
