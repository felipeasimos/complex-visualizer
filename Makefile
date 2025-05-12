.PHONY: all build serve vercel-build
all: build serve
build:
	RUSTC_WRAPPER=sccache wasm-pack build --out-dir www/pkg --target web
serve: build
	cd www && npm install && npm run build && npm run dev
