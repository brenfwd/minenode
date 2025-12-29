CARGO ?= cargo
BUN ?= bun

BUN_TARGET = build

.SUFFIXES:

build: configure
	cd crates/minenode_napi && $(BUN) run $(BUN_TARGET)
.PHONY: build

release: BUN_TARGET = build:release
release: build configure
	cd js && $(BUN) build --compile --outfile=minenode-bundle --target=bun index.ts
.PHONY: release

configure:
	cargo check
	cd crates/minenode_napi && $(BUN) install
	cd js && $(BUN) install
.PHONY: configure

clean:
	$(CARGO) clean
	cd crates/minenode_napi && rm -rf node_modules/ dist/
	cd js && rm -rf node_modules/ minenode-bundle
.PHONY: clean
