.DEFAULT_GOAL := test

wasm_src/Cargo.lock:
	cd wasm_src && cargo update

wasm_src/pkg/macro_railroad_ext.wasm: wasm_src/Cargo.lock wasm_src/Cargo.toml wasm_src/build.rs wasm_src/src/lib.rs
	cd wasm_src && wasm-pack build -t no-modules && cp pkg/macro_railroad_ext_bg.wasm pkg/macro_railroad_ext.wasm

wasm_src/pkg/macro_railroad_ext.js: wasm_src/pkg/macro_railroad_ext.wasm

wasm:
	mkdir $@

wasm/macro_railroad_ext.wasm: wasm_src/pkg/macro_railroad_ext.wasm | wasm
	cp $< $@

content_scripts/macro_railroad_ext.js: wasm_src/pkg/macro_railroad_ext.js
	cp $< $@

packed.zip: manifest.json content_scripts/macro_railroad_ext.js content_scripts/main.js icons/icon48.png icons/icon96.png options/options.html wasm/macro_railroad_ext.wasm
	zip $@ $?

.PHONY: clean
clean:
	-rm packed.zip wasm_src/Cargo.lock wasm/macro_railroad_ext.wasm content_scripts/macro_railroad_ext.js
	-rm -r wasm_src/pkg
	-rmdir wasm
	cd wasm_src && cargo clean
	cd chrome_helper && cargo clean
