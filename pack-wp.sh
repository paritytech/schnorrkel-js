#!/usr/bin/env node
console.log('\t+++ Fixing webpack imports.');
const fs = require('fs');

fs.writeFileSync('./pkg/schnorrkel.wp.bg.js', `const imports = {};
imports['./schnorrkel_js'] = require('./schnorrkel.wp');
WebAssembly.instantiateStreaming(fetch('./schnorrkel_js_bg.wasm'), imports)
.then(results => {
    console.info('++ schnorrkel-js wasm blob loaded.')
    module.exports = results.instance.exports
});`);