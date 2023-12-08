#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "mygame" \
    ./target/wasm32-unknown-unknown/release/mygame.wasm

butler push teavm/build/dist/webapp lavaeater/jam-packed-christmas:html
