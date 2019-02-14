#!/usr/bin/env bash

# Compile the latest version; update ./pkg and readme files in it.
wasm-pack build --target browser

# Fix the name 
sed -i -e 's/schnorrkel-js/@parity\/schnorrkel-js/g' pkg/package.json

# Run the script to fix node/browser import support
./pack-node.sh

# publish 
# wasm-pack publish