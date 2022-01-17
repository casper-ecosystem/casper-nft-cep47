prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p cep47 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/cep47-token.wasm 2>/dev/null | true

test-only:
	cargo test -p cep47-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm cep47-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf cep47-tests/wasm/*.wasm
