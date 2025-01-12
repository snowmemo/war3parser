wasm:
    rm -rf dist
    cd war3parser && wasm-pack build --target web --out-dir ../dist

wasm-publish:
    cd dist && npm publish
