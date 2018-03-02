DEBUG_DIR =  target/debug
RELEASE_DIR =  target/release
WASM_DIR =  target/wasm32-unknown-unknown/release
DEPLOY_DIR = dist
PARCEL_BIN = parcel

.PHONY: all debug release clean

all: debug

debug: builds buildc

release: buildsr buildc

builds:
	cd server ; \
		cargo build ; \
		cp -f $(DEBUG_DIR)/server ../serve

buildsr:
	cd server ; \
		cargo build --release ; \
		cp -f $(RELEASE_DIR)/server ../serve

buildc:
	cd client ; \
		parcel build --out-dir ../$(DEPLOY_DIR) --public-url . src/index.html

clean: cleans cleanc

cleans:
	rm -f serve
	cd server ; \
		cargo clean

cleanc:
	rm -rf $(DEPLOY_DIR)/
	cd client ; \
		cargo clean ; \
		rm -rf .cache ; \
		rm -rf node_modules
