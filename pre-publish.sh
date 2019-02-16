#!/usr/bin/env bash

# Compile the latest version; update ./pkg and readme files in it.
wasm-pack build --target nodejs
mv ./pkg/schnorrkel_js.js ./pkg/schnorrkel.node.js

wasm-pack build --target browser
mv ./pkg/schnorrkel_js.js ./pkg/schnorrkel.wp.js

# Fix the name 
sed -i -e 's/schnorrkel-js/@parity\/schnorrkel-js/g' pkg/package.json

# Run the script to fix node/browser import support
./pack-node.sh

# publish 
# wasm-pack publish