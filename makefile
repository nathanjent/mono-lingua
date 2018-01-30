DEBUG_DIR =  target/debug
RELEASE_DIR =  target/release
WASM_DIR =  target/wasm32-unknown-unknown/release
DEPLOY_DIR = dist

.PHONY: all debug release clean

all: debug

debug: buildd buildf

buildd:
	mkdir -p dist/
	cd server ; \
		cargo build ; \
		cp -f $(DEBUG_DIR)/server ../serve

release: buildr buildf

buildr:
	mkdir -p dist/
	cd server ; \
		cargo build --release ; \
		cp -f $(RELEASE_DIR)/server ../serve

buildf:
	cd frontend ; \
		mkdir -p dist/ ; \
		cargo build --release ; \
		cp -f $(WASM_DIR)/frontend.wasm src/ ; \
		parcel build src/index.html ; \
		mv -u $(DEPLOY_DIR)/* ../$(DEPLOY_DIR)/

clean: cleanb cleanf

cleanb:
	rm -rf $(DEPLOY_DIR)/
	rm -f serve
	cd server ; \
		cargo clean

cleanf:
	cd frontend ; \
		cargo clean ; \
		rm -rf $(DEPLOY_DIR)/
