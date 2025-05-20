.PHONY: all run-dev release-web release
all: build serve
build:
	wasm-pack build --out-dir www/pkg --target web
serve: build
	cd www && npm install && npm run build && npm run dev
