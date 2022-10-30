import { sqrt } from 'mathjs'
import * as tf from '@tensorflow/tfjs';

console.log(sqrt(-4).toString()) // 2i
console.log('inside webpack index.js')
const url = browser.runtime.getURL('model/output/model.json');
const model = await tf.loadLayersModel(url);
model.summary();

const IMAGE_SIZE = 100;
const normal = tf.randomNormal([1, IMAGE_SIZE, IMAGE_SIZE, 3]);
const r = model.predict(normal);

console.log(r.toString());
