#!/bin/bash

rm -rf ./out
cargo build --target=x86_64-pc-windows-msvc --release

mkdir -p ./out/assets
cp -r ./assets/* ./out/assets/

butler push out lavaeater/santa-fighting-christmas:win
