#!/usr/bin/env node
const fs = require('fs');

console.log('\t+++ Fixing node imports.');
fs.writeFileSync('./pkg/schnorrkel.node.bg.js', `
const path = require('path').join(__dirname, 'schnorrkel_js_bg.wasm');
const bytes = require('fs').readFileSync(path);
let imports = {};
imports['./schnorrkel_js'] = require('./schnorrkel.node');

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
`);