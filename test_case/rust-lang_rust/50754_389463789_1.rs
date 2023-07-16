js
const fs = require('fs');
let buf = fs.readFileSync('lib.wasm');
let m = new WebAssembly.Module(buf);
let inst = new WebAssembly.Instance(m, {});
console.log(inst.exports.test());
