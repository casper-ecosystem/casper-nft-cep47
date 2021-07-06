prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p dragons-nft --target wasm32-unknown-unknown

test-only:
	cargo test -p cep47-logic
	cargo test -p dragons-nft-tests

copy-wasm-file-to-test:
	mkdir -p dragons-nft-tests/wasm
	cp target/wasm32-unknown-unknown/release/*.wasm dragons-nft-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all

check-lint: clippy
	cargo fmt --all -- --check

format:
	cargo fmt --all

lint: clippy format
	
clean:
	cargo clean
	rm -rf dragons-nft-tests/wasm/*.wasm
