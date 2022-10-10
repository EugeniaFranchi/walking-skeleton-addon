(async () => {
    await load();
})();

async function load() {
    await wasm_bindgen(chrome.runtime.getURL('wasm/macro_railroad_ext.wasm'));
    wasm_bindgen.funcion();
    wasm_bindgen.add(1,2);
}
