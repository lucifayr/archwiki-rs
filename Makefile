NPMJS_SCOPE=lucifayr
WASM_WEB_PKG_NAME = archwiki-web
WASM_WEB_PKG_DIR = wasm-web/pkg
WASM_NODEJS_PKG_NAME = archwiki-node
WASM_NODEJS_PKG_DIR = wasm-nodejs/pkg

cli : 
	cargo build --release

wasm-web :
	wasm-pack build --release -t web -s $(NPMJS_SCOPE) --out-name $(WASM_WEB_PKG_NAME) --out-dir $(WASM_WEB_PKG_DIR)  ./ --features wasm-web --no-default-features
	jq -s '.[0] * .[1]'  $(WASM_WEB_PKG_DIR)/package.json $(WASM_WEB_PKG_NAME).json | sponge $(WASM_WEB_PKG_DIR)/package.json
	cp $(WASM_WEB_PKG_NAME)-README.md $(WASM_WEB_PKG_DIR)/README.md

wasm-nodejs :
	wasm-pack build --release -t nodejs -s $(NPMJS_SCOPE) --out-name $(WASM_NODEJS_PKG_NAME) --out-dir $(WASM_NODEJS_PKG_DIR) ./ --features wasm-nodejs --no-default-features
	jq -s '.[0] * .[1]'  $(WASM_NODEJS_PKG_DIR)/package.json $(WASM_NODEJS_PKG_NAME).json | sponge $(WASM_NODEJS_PKG_DIR)/package.json
	cp $(WASM_NODEJS_PKG_NAME)-README.md $(WASM_NODEJS_PKG_DIR)/README.md

wasm-web-publish : wasm-web
	wasm-pack pack $(WASM_WEB_PKG_DIR)

wasm-nodejs-publish : wasm-nodejs
	wasm-pack pack $(WASM_NODEJS_PKG_DIR)

wasm : wasm-web wasm-nodejs

wasm-publish : wasm-web-publish wasm-nodejs-publish
