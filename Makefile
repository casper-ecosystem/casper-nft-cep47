prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p test-contracts --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/dragons-nft.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/owning-contract.wasm 2>/dev/null | true

test-only:
	cargo test -p cep47-logic
	cargo test -p cep47-tests

copy-wasm-file-to-test:
	mkdir -p cep47-tests/wasm
	cp target/wasm32-unknown-unknown/release/*.wasm cep47-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -A clippy::ptr_arg

check-lint: clippy
	cargo fmt --all -- --check

format:
	cargo fmt --all

lint: clippy format
	
clean:
	cargo clean
	rm -rf cep47-tests/wasm/*.wasm
