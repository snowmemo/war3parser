wasm:
    rm -rf dist
    wasm-pack build packages/lib --target web --out-dir ../../dist --scope wesleyel

wasm-publish:
    cd dist && npm publish --access=public
