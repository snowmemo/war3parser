clean-wasm:
    rm -rf dist
    mkdir -p dist

build-wasm: clean-wasm
    wasm-pack build {{justfile_directory()}}/packages/wasm --target web --out-dir ../../dist --scope wesleyel

build-lib:
    cd {{justfile_directory()}}/packages/lib && cargo build

build-cli:
    cd {{justfile_directory()}}/packages/cli && cargo build

build: build-wasm build-lib build-cli

lint:
    cd {{justfile_directory()}} && cargo fmt --all
    cd {{justfile_directory()}} && cargo clippy --all-targets --all-features -- -D warnings

test:
    cd {{justfile_directory()}} && cargo test --all-targets --all-features
