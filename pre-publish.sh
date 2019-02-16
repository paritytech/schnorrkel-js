#!/usr/bin/env bash

# Compile the latest version; update ./pkg and readme files in it.
wasm-pack build --target nodejs
mv ./pkg/schnorrkel_js.js ./pkg/schnorrkel.node.js
cp ./pkg/schnorrkel_js_bg.js ./pkg/schnorrkel.node.bg.js
sed -i -e 's/schnorrkel_js_bg/schnorrkel.node.bg/g' ./pkg/schnorrkel.node.js


wasm-pack build --target browser
mv ./pkg/schnorrkel_js.js ./pkg/schnorrkel.wp.js
mv ./pkg/schnorrkel_js_bg.js ./pkg/schnorrkel.wp.bg.js
sed -i -e 's/schnorrkel_js_bg/schnorrkel.wp.bg/g' ./pkg/schnorrkel.wp.js

# Replace package.json file.
cp ./package.json ./pkg/package.json

# Run the script to fix node/browser import support
# ./pack-node.sh

# publish 
# wasm-pack publish