#!/usr/bin/env node

console.log('+++ Fixing nodejs imports.\n');

const fs = require('fs');
const buffer = fs.readFileSync('./pkg/schnorrkel_js_bg.wasm');

fs.writeFileSync('./pkg/schnorrkel_js_bg.js', `
const schnorrkel_js = require('./schnorrkel_js');
const imports = {};
imports['./schnorrkel_js'] = schnorrkel_js;
const bytes = Buffer.from('${buffer.toString('base64')}', 'base64');
const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);

module.exports = wasmInstance.exports;
`);
