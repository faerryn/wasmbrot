#!/bin/sh

set -ex

wasm-pack build --target web
cp pkg/wasmbrot.js .
cp pkg/wasmbrot_bg.wasm .

python3 -m http.server
