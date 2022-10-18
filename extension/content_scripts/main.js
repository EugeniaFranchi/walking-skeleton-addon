(async () => {
    await load();
})();

async function load() {
    await wasm_bindgen(chrome.runtime.getURL('wasm/macro_railroad_ext.wasm'));
    wasm_bindgen.funcion();
    wasm_bindgen.add(1,2);
    let images = document.images
    var imgList = [];
    for(var i = 0; i < images.length; i++) {
        const response = await fetch(images[i].src)
                                .then((response) => response.arrayBuffer())
        const data = new Uint8Array(response);
        console.log("data", data)
        imgList.push(response);
    }
      
    wasm_bindgen.count(imgList);
}
