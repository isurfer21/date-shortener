#!/bin/sh
echo "Generate debug builds"

cargo build 
file ./target/debug/ds

cargo build --target wasm32-wasi
file ./target/wasm32-wasi/debug/ds.wasm

cargo build --target wasm32-unknown-emscripten
file ./target/wasm32-unknown-emscripten/debug/ds.wasm

echo "Generate release builds"

cargo build --release
file ./target/release/ds

cargo build --target wasm32-wasi --release
file ./target/wasm32-wasi/release/ds.wasm

cargo build --target wasm32-unknown-emscripten --release
file ./target/wasm32-unknown-emscripten/release/ds.wasm

echo "Done!"