#!/bin/sh

cargo build --release --target wasm32-unknown-unknown &&
cp ./target/wasm32-unknown-unknown/release/trijam.wasm ./html/game.wasm &&
cp -r ./assets ./html/ &&
python3 -m http.server --directory ./html
