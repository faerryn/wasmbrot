#!/bin/sh

set -ex

wasm-pack build --target web --release
cp pkg/wasmbrot.js .
cp pkg/wasmbrot_bg.wasm .
