#! /bin/env bash

cargo build --release --target wasm32-wasip2
wasmtime serve -Scli target/wasm32-wasip2/release/wasm_nice_pages.wasm
