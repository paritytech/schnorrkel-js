#!/usr/bin/env node
console.log('\t+++ Fixing webpack imports.');
const fs = require('fs');
const buffer = fs.readFileSync('./pkg/schnorrkel_js_bg.wasm');

fs.writeFileSync('./pkg/schnorrkel.wp.bg.js', `
const imports = {};
imports['./schnorrkel_js'] = require('./schnorrkel.wp');

const bytes = Buffer.from('${buffer.toString('base64')}', 'base64');

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
`);