#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "santafightingchristmas" \
    ./target/wasm32-unknown-unknown/release/bevy-jam-4.wasm

cp -r ./html/* ./out/
mkdir -p ./out/assets
cp -r ./assets/* ./out/assets/


butler push out lavaeater/santa-fighting-christmas:html
