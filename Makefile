SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rulesNPMJS_SCOPE=lucifayr

WASM_WEB_PKG_NAME = archwiki-web
WASM_WEB_PKG_DIR = wasm-web/pkg
WASM_NODEJS_PKG_NAME = archwiki-node
WASM_NODEJS_PKG_DIR = wasm-nodejs/pkg
WASM_BUNDLER_PKG_NAME = archwiki-bundler
WASM_BUNDLER_PKG_DIR = wasm-bundler/pkg

CLI_VERSION=$(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[]  | select(.name == "archwiki-rs") | .version')
WASM_WEB_VERSION=$(shell jq -r '.version' $(WASM_WEB_PKG_NAME).json)
WASM_NODEJS_VERSION=$(shell jq -r '.version' $(WASM_NODEJS_PKG_NAME).json)
WASM_BUNDLER_VERSION=$(shell jq -r '.version' $(WASM_BUNDLER_PKG_NAME).json)

manual :
	ronn --roff man/ronn/*.ronn -o man/man

check-version-cli :
	(! git diff --quiet Cargo.toml && git diff Cargo.toml | grep  '+version = ' -B 2 | head -n 1 | grep -q ' name = "archwiki-rs"') || printf "WARNING: version change not detected in diff\n"

check-version-wasm-web :
	(! git diff --quiet $(WASM_WEB_PKG_NAME).json && git diff $(WASM_WEB_PKG_NAME).json | grep -q '+  "version": ') || printf "WARNING: version change not detected in diff\n"

check-version-wasm-nodejs :
	(! git diff --quiet $(WASM_NODEJS_PKG_NAME).json && git diff $(WASM_NODEJS_PKG_NAME).json | grep -q '+  "version": ') || printf "WARNING: version change not detected in diff\n"

check-version-wasm-bundler :
	(! git diff --quiet $(WASM_BUNDLER_PKG_NAME).json && git diff $(WASM_BUNDLER_PKG_NAME).json | grep -q '+  "version": ') || printf "WARNING: version change not detected in diff\n"

build-wasm-web:
	wasm-pack build --release -t web --out-name $(WASM_WEB_PKG_NAME) --out-dir $(WASM_WEB_PKG_DIR)  ./ --features wasm-web --no-default-features
	jq -s '.[0] * .[1]'  $(WASM_WEB_PKG_DIR)/package.json $(WASM_WEB_PKG_NAME).json | sponge $(WASM_WEB_PKG_DIR)/package.json
	cp $(WASM_WEB_PKG_NAME)-README.md $(WASM_WEB_PKG_DIR)/README.md

build-wasm-nodejs:
	wasm-pack build --release -t nodejs --out-name $(WASM_NODEJS_PKG_NAME) --out-dir $(WASM_NODEJS_PKG_DIR) ./ --features wasm-nodejs --no-default-features
	jq -s '.[0] * .[1]'  $(WASM_NODEJS_PKG_DIR)/package.json $(WASM_NODEJS_PKG_NAME).json | sponge $(WASM_NODEJS_PKG_DIR)/package.json
	cp $(WASM_NODEJS_PKG_NAME)-README.md $(WASM_NODEJS_PKG_DIR)/README.md

build-wasm-bundler:
	wasm-pack build --release -t bundler --out-name $(WASM_BUNDLER_PKG_NAME) --out-dir $(WASM_BUNDLER_PKG_DIR) ./ --features wasm-bundler --no-default-features
	jq -s '.[0] * .[1]'  $(WASM_BUNDLER_PKG_DIR)/package.json $(WASM_BUNDLER_PKG_NAME).json | sponge $(WASM_BUNDLER_PKG_DIR)/package.json
	cp $(WASM_BUNDLER_PKG_NAME)-README.md $(WASM_BUNDLER_PKG_DIR)/README.md

build-wasm: build-wasm-web build-wasm-nodejs build-wasm-bundler

bump-cli : check-version-cli manual
	cargo build --release
	git add Cargo.toml Cargo.lock
	git commit -m "bump cli version to $(CLI_VERSION)" || printf "WARNING: nothing to commit for cli version bump"
	git tag "v$(CLI_VERSION)" || printf "WARNING: tag 'v$(CLI_VERSION)' already exists"

bump-wasm-web : check-version-wasm-web build-wasm-web
	wasm-pack pack $(WASM_WEB_PKG_DIR)
	git add $(WASM_WEB_PKG_NAME).json
	git commit -m "bump wasm-web version to $(WASM_WEB_VERSION)" || printf "WARNING: nothing to commit for wasm-web version bump"
	git tag "wasm-web-v$(WASM_WEB_VERSION)" || printf "WARNING: tag 'wasm-web-v$(WASM_WEB_VERSION)' already exists"

bump-wasm-nodejs : check-version-wasm-nodejs build-wasm-nodejs
	wasm-pack pack $(WASM_NODEJS_PKG_DIR)
	git add $(WASM_NODEJS_PKG_NAME).json
	git commit -m "bump wasm-nodejs version to $(WASM_NODEJS_VERSION)" || printf "WARNING: nothing to commit for wasm-nodejs version bump"
	git tag "wasm-nodejs-v$(WASM_NODEJS_VERSION)" || printf "WARNING: tag 'wasm-nodejs-v$(WASM_NODEJS_VERSION)' already exists"

bump-wasm-bundler : check-version-wasm-bundler build-wasm-bundler
	wasm-pack pack $(WASM_BUNDLER_PKG_DIR)
	git add $(WASM_BUNDLER_PKG_NAME).json
	git commit -m "bump wasm-bundler version to $(WASM_BUNDLER_VERSION)" || printf "WARNING: nothing to commit for wasm-bundler version bump"
	git tag "wasm-bundler-v$(WASM_BUNDLER_VERSION)" || printf "WARNING: tag 'wasm-bundler-v$(WASM_BUNDLER_VERSION)' already exists"

bump-wasm : bump-wasm-web bump-wasm-nodejs bump-wasm-bundler

push-tags:
	git push --follow-tags
	git push --follow-tags github

artifacts-cli :
	mkdir -p artifacts
	tar -czf artifacts/man.tar.gz man/man

publish-no-bump-cli : artifacts-cli
	cargo publish
	glab release create "v$(CLI_VERSION)" -n "cli v$(CLI_VERSION)" './artifacts/man.tar.gz#manual pages'
	gh release create "v$(CLI_VERSION)" -t "cli v$(CLI_VERSION)" './artifacts/man.tar.gz#manual pages' --generate-notes --target main --latest=true

publish-no-bump-wasm-web :
	wasm-pack publish $(WASM_WEB_PKG_DIR)
	glab release create "wasm-web-v$(WASM_WEB_VERSION)" -n "wasm web v$(WASM_WEB_VERSION)"
	gh release create "wasm-web-v$(WASM_WEB_VERSION)" -t "wasm web v$(WASM_WEB_VERSION)" --generate-notes --target main --latest=false

publish-no-bump-wasm-nodejs :
	wasm-pack publish $(WASM_NODEJS_PKG_DIR)
	glab release create "wasm-nodejs-v$(WASM_NODEJS_VERSION)" -n "wasm nodejs v$(WASM_NODEJS_VERSION)"
	gh release create "wasm-nodejs-v$(WASM_NODEJS_VERSION)" -t "wasm nodejs v$(WASM_NODEJS_VERSION)" --generate-notes --target main --latest=false

publish-no-bump-wasm-bundler :
	wasm-pack publish $(WASM_BUNDLER_PKG_DIR)
	glab release create "wasm-bundler-v$(WASM_BUNDLER_VERSION)" -n "wasm bundler v$(WASM_BUNDLER_VERSION)"
	gh release create "wasm-bundler-v$(WASM_BUNDLER_VERSION)" -t "wasm bundler v$(WASM_BUNDLER_VERSION)" --generate-notes --target main --latest=false

publish-no-bump-wasm : publish-no-bump-wasm-web publish-no-bump-wasm-nodejs publish-no-bump-wasm-bundler

publish-cli : bump-cli push-tags publish-no-bump-cli
publish-wasm-web : bump-wasm-web push-tags publish-no-bump-wasm-web
publish-wasm-nodejs : bump-wasm-nodejs push-tags publish-no-bump-wasm-nodejs
publish-wasm-bundler : bump-wasm-bundler push-tags publish-no-bump-wasm-bundler
publish-wasm : publish-wasm-web publish-wasm-nodejs publish-wasm-bundler
