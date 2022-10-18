(async () => {
    await load();
})();

async function load() {
    await wasm_bindgen(chrome.runtime.getURL('wasm/macro_railroad_ext.wasm'));
    wasm_bindgen.funcion();
    wasm_bindgen.add(1,2);
    let images = document.images
    var srcList = [];
    for(var i = 0; i < images.length; i++) {
        srcList.push(images[i].src);
        console.log(srcList);
    }
      
    wasm_bindgen.count(srcList);
}
