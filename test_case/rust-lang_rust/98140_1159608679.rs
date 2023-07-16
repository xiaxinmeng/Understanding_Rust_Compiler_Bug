plain
---- [ui] src/test/ui/numbers-arithmetic/promoted_overflow.rs stdout ----

error: error pattern ' overflow' not found!
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/promoted_overflow/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at <anonymous>:wasm-function[62]:0x432b
    at <anonymous>:wasm-function[60]:0x4317
    at <anonymous>:wasm-function[59]:0x42e7
    at <anonymous>:wasm-function[48]:0x39d3
    at <anonymous>:wasm-function[47]:0x3942
    at <anonymous>:wasm-function[54]:0x3f7e
    at <anonymous>:wasm-function[118]:0x521f
    at <anonymous>:wasm-function[123]:0x56ed
    at <anonymous>:wasm-function[4]:0x257
    at <anonymous>:wasm-function[0]:0x1b8
    at <anonymous>:wasm-function[1]:0x1da
    at <anonymous>:wasm-function[42]:0x3298
    at main (<anonymous>:wasm-function[5]:0x28e)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47


---- [ui] src/test/ui/panics/unique-panic.rs stdout ----


error: error pattern ' panic' not found!
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/unique-panic/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at <anonymous>:wasm-function[69]:0x4478
    at <anonymous>:wasm-function[67]:0x4464
    at <anonymous>:wasm-function[66]:0x4434
    at <anonymous>:wasm-function[2]:0x239
    at <anonymous>:wasm-function[1]:0x1fe
    at <anonymous>:wasm-function[5]:0x2c3
    at <anonymous>:wasm-function[10]:0x39f
    at <anonymous>:wasm-function[3]:0x245
    at <anonymous>:wasm-function[4]:0x267
    at <anonymous>:wasm-function[48]:0x33e0
    at main (<anonymous>:wasm-function[11]:0x3d6)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47



failures:
