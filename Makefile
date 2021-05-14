prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p contract --target wasm32-unknown-unknown

test-only:
	cargo test -p tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/contract.wasm tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all
	
clean:
	cargo clean
	rm -rf tests/wasm/contract.wasm
