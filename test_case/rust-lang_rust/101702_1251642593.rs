plain
doc tests for: /checkout/src/doc/rustc/src/tests/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 1.491 seconds
Generating lint docs (x86_64-unknown-linux-gnu)
internal/modules/cjs/loader.js:818
  throw err;

Error: Cannot find module '/checkout/obj/build/x86_64-unknown-linux-gnu/doc/search1.66.0.js'
Require stack:
- /checkout/src/tools/rustdoc-js/tester.js
- /checkout/src/tools/rustdoc-js/tester.js
    at Function.Module._resolveFilename (internal/modules/cjs/loader.js:815:15)
    at Function.Module._load (internal/modules/cjs/loader.js:667:27)
    at Module.require (internal/modules/cjs/loader.js:887:19)
    at require (internal/modules/cjs/helpers.js:74:18)
    at loadSearchJS (/checkout/src/tools/rustdoc-js/tester.js:313:26)
    at main (/checkout/src/tools/rustdoc-js/tester.js:391:26)
    at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:418:14)
    at Module._compile (internal/modules/cjs/loader.js:999:30)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:1027:10)
    at Module.load (internal/modules/cjs/loader.js:863:32) {
  code: 'MODULE_NOT_FOUND',
  requireStack: [ '/checkout/src/tools/rustdoc-js/tester.js' ]
Build completed unsuccessfully in 0:26:42
