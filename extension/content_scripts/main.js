(async () => {
    await load();
})();

function readFile(_path, _cb) {
    fetch(_path, { mode: 'same-origin' })   // <-- important
        .then(function (_res) {
            return _res.blob();
        })
        .then(function (_blob) {
            var reader = new FileReader();
            reader.addEventListener("loadend", function () {
                _cb(this.result);
            });
            reader.readAsArrayBuffer(_blob);
        });
};

async function load() {
    document.body.style.border = "5px solid red";
    await wasm_bindgen(chrome.runtime.getURL('wasm/macro_railroad_ext.wasm'));
    wasm_bindgen.funcion();
    wasm_bindgen.add(1, 2);

    readFile(chrome.runtime.getURL('classifier/model/model.onnx'), function (_res) {
        console.log(_res); // <--  result (file content)
        var bytes = new Uint8Array(_res);
        console.log(bytes);

        readFile(chrome.runtime.getURL('classifier/example.jpg'), function (img) {
            console.log(img); // <--  result (file content)
            var pix = new Uint8Array(img);
            console.log(pix);
            wasm_bindgen.infer(bytes, pix, 100, 100);
        });
    });
}
