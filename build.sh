#!/bin/sh

set -ex

wasm-pack build --target web --release
rm wasmbrot.js
rm wasmbrot_bg.wasm
cp pkg/wasmbrot.js .
cp pkg/wasmbrot_bg.wasm .
