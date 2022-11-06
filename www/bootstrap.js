import * as tf from '@tensorflow/tfjs';
import getPixels from 'get-pixels';
import util from 'util';
const { add } = wasm_bindgen;

document.body.style.border = "5px solid red";
console.log('from bootstrap')

async function getImages() {
    let images = document.images
    var imgList = [];
    const asyncGetPixels = util.promisify(getPixels);
    console.log("Nro de imágenes a procesar:", images.length);
    for(var i = 0; i < images.length; i++) {
        const pixels = await asyncGetPixels(images[i].src, (err, pixels) => pixels)
        const rgbaTens3d = tf.tensor3d(pixels.data, [pixels.shape[0], pixels.shape[1], 4])
        const rgbTens3d= tf.slice3d(rgbaTens3d, [0, 0, 0], [-1, -1, 3]) // strip alpha channel
        const smallImg = tf.image.resizeBilinear(rgbTens3d, [100, 100]);
        const tensor = smallImg.reshape([1, 100, 100, 3])
        imgList.push(tensor);
    }
    return imgList
}

async function classifyImages(imgList) {
    // Carga del modelo
    const model = await tf.loadLayersModel(browser.runtime.getURL('classifier/model/model.json'));
    model.summary();

    // Predicción
    imgList.forEach(img => {
        const r = model.predict(img);
        console.log("Prediction: ", r.toString());
    });
}

async function run() {
    await wasm_bindgen(browser.runtime.getURL('pkg/wasmaddon_bg.wasm'));

    console.log("--INICIO PROCESO 1: IMAGEN -> TENSOR--");
    const imgList = await getImages();
    console.log("--FIN PROCESO 1--");

    console.log("--INICIO PROCESO 2: TENSOR -> PREDICCIÓN--");
    await classifyImages(imgList);
    console.log("--FIN PROCESO 2--");

    // TODO: Pintar de rojo las imágenes peligrosas

    
    // const IMAGE_SIZE = 100;
    // const normal = tf.randomNormal([1, IMAGE_SIZE, IMAGE_SIZE, 3]);
    // const r = model.predict(normal);

}

run();