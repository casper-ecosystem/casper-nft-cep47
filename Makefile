prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo +nightly build --release -p cep47 --target wasm32-unknown-unknown

test-only:
	cargo +nightly test --workspace

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/cep47.wasm tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo +nightly clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

format:
	cargo fmt --all

lint: clippy format
	
clean:
	cargo clean
	rm -rf tests/wasm/cep47.wasm
