#!/bin/sh
wasm-pack build --dev --out-name v1 --target web && python3 -m http.server