#!/usr/bin/env bash

# Compile the latest version; update ./pkg and readme files in it.
wasm-pack build --target nodejs --release
mv ./pkg/schnorrkel_js.js ./pkg/schnorrkel.node.js
mv ./pkg/schnorrkel_js_bg.js ./pkg/schnorrkel.node.bg.js
sed -i -e 's/schnorrkel_js_bg/schnorrkel.node.bg/g' ./pkg/schnorrkel.node.js
./pack-node.sh

wasm-pack build --target browser --release
mv ./pkg/schnorrkel_js.js ./pkg/schnorrkel.wp.js
sed -i -e 's/schnorrkel_js_bg/schnorrkel.wp.bg.js/g' ./pkg/schnorrkel.wp.js
./pack-wp.sh
# will most likely cause an error saying wasm blobl is larger than 4KB
# ./pack-wp-raw.sh

# Replace package.json file.
cp ./package.json ./pkg/package.json

# publish 
# wasm-pack publish

# replace this at the top of the .wp.js file 
# import get_wasm from './schnorrkel.wp.bg.js';
# var wasm = get_wasm()
# // this line is a gamble...
# setTimeout(() => wasm = get_wasm(), 500)
# setTimeout(() => wasm = get_wasm(), 1000)
# setTimeout(() => wasm = get_wasm(), 1500)