import * as tf from '@tensorflow/tfjs';
import getPixels from 'get-pixels';
import util from 'util';
import axios from 'axios';
const { add } = wasm_bindgen;

document.body.style.border = "5px solid red";
console.log('from bootstrap')

async function getImagesAndPredict(model) {
    let stego = 0;
    let images = document.images
    const asyncGetPixels = util.promisify(getPixels);
    console.log("Nro de imágenes a procesar:", images.length);
    for (var i = 0; i < images.length; i++) {
        console.log(`Procesando imagen ${i}`);
        // OBTENER IMAGEN
        const pixels = await asyncGetPixels(images[i].src, (err, pixels) => pixels)

        // PREDICCIÓN
        // Uint8Array -> Tensor 3D RGBA [x,y,4]
        const rgbaTens3d = tf.tensor3d(pixels.data, [pixels.shape[0], pixels.shape[1], 4])
        // Tensor 3D RGBA [x,y,4] -> Tensor 3D RGB [x,y,3]
        const rgbTens3d = tf.slice3d(rgbaTens3d, [0, 0, 0], [-1, -1, 3])
        // Tensor 3D RGB [x,y,3] -> Tensor 3D RGB [100,100,3]
        const smallImg = tf.image.resizeBilinear(rgbTens3d, [100, 100]);
        // Tensor 3D RGB [100,100,3] -> Tensor 4D RGB [1,100,100,3]
        const tensor = smallImg.reshape([1, 100, 100, 3])
        const prediction = model.predict(tensor).dataSync();
        console.log("Prediction: ", prediction);

        // PINTAR IMAGEN
        if (prediction[0] >= 0.7) {
            images[i].style.filter = "blur(2px) grayscale(100%) brightness(40%) sepia(100%) hue-rotate(-50deg) saturate(600%) contrast(0.8)";
            stego += 1;
        }
    }
    return stego
}

async function run() {
    await wasm_bindgen(browser.runtime.getURL('pkg/wasmaddon_bg.wasm'));

    // Carga del modelo
    const model = await tf.loadLayersModel(browser.runtime.getURL('classifier/model/model.json'));
    model.summary();


    console.log("--INICIO PROCESO: IMAGEN -> PREDICCIÓN--");
    let stego = await getImagesAndPredict(model);
    console.log("--FIN PROCESO--");

    console.log(`sending counter ${stego} from page ${document.domain}`);
    await axios.post('http://localhost:5000/counter', { site: document.domain, stego: stego, total: document.images.length })
        .then((response) => {
            console.log(response);
        }).then(error => {
            console.log(error);
        });


    // TODO: Pintar de rojo las imágenes peligrosas


    // const IMAGE_SIZE = 100;
    // const normal = tf.randomNormal([1, IMAGE_SIZE, IMAGE_SIZE, 3]);
    // const r = model.predict(normal);

}

run();