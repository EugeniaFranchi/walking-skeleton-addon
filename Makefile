all: model pkg www

model:
	jupyter execute classifier/classifier.ipynb
	tensorflowjs_converter --input_format keras classifier/model/model.h5 classifier/model

pkg:
	wasm-pack build --dev --target no-modules

www:
	cd www && npm install && npm run build

web-ext:
	mkdir -p tmp
	TMPDIR=./tmp web-ext run

.PHONY: www pkg all model