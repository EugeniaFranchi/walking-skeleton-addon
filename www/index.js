(async () => {
  await load();
})();

async function load() {
  document.body.style.border = "5px solid red";
  console.log('here')
  //const x = await wasm_bindgen(browser.runtime.getURL("wasm/wasm_game_of_life.wasm"));
  //console.log("x:", x)
  console.log('here2')
  //console.log(wasm_bindgen.add(1,2));
}

