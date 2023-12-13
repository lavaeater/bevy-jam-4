#!/bin/bash

rm -rf ./out
cargo build --release

mkdir -p ./out/assets
cp -r ./assets/* ./out/assets/

butler push out lavaeater/santa-fighting-christmas:mac
