#!/usr/bin/env bash

# Compile the latest version; update ./pkg and readme files in it.
wasm-pack build --target nodejs --scope substrate

# Run the script to fix node/browser import support
./pack-node.sh

# publish 
# wasm-pack publish