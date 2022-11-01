import * as tf from '@tensorflow/tfjs';
const { add } = wasm_bindgen;

document.body.style.border = "5px solid red";
console.log('from bootstrap')

async function run() {
    await wasm_bindgen(browser.runtime.getURL('pkg/wasmaddon_bg.wasm'));
    const result = add(1, 2);
    console.log(`1 + 2 = ${result}`);

    const model = await tf.loadLayersModel(browser.runtime.getURL('classifier/model/model.json'));
    model.summary();

    const IMAGE_SIZE = 100;
    const normal = tf.randomNormal([1, IMAGE_SIZE, IMAGE_SIZE, 3]);
    const r = model.predict(normal);

    console.log(r.toString());
}

run();