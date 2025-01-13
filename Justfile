clean:
    rm -rf dist
    mkdir -p dist

wasm-web:
    wasm-pack build packages/lib --target web --out-dir ../../dist/web --scope wesleyel

publish-web:
    cd dist/web && npm publish --access=public

build: clean wasm-web

publish: publish-web