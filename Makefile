.SUFFIXES:

all: debug
.PHONY: all

debug: harness-debug
.PHONY: debug

harness-debug:
	cargo build --bin minenode_harness
.PHONY: harness-debug

# minenode_napi uses napi-rs and must be built using its CLI tool
napi-debug:
	cd crates/minenode_napi && bun install --frozen-lockfile && bun run build
.PHONY: napi-debug

js-debug: napi-debug
	cd js && bun install --frozen-lockfile && bun run build
.PHONY: js-debug

watch: harness-debug
	@cargo install --locked bacon
	bacon run -- --bin minenode_harness
.PHONY: watch

clean:
	cargo clean
	cd crates/minenode_napi && rm -rf node_modules index*.node index.js index.d.ts
	cd js && rm -rf node_modules
.PHONY: clean
