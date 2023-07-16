
/home/the8472/workspace/rust/src/etc/wasm32-shim.js:17
let m = new WebAssembly.Module(buffer);
        ^

CompileError: WebAssembly.Module(): Compiling function #3:"_ZN11issue_230364main17hf6fa4aae1344ccadE" failed: Invalid opcode 0xff @+874
    at Object.<anonymous> (/home/the8472/workspace/rust/src/etc/wasm32-shim.js:17:9)
    at Module._compile (node:internal/modules/cjs/loader:1097:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1149:10)
    at Module.load (node:internal/modules/cjs/loader:975:32)
    at Function.Module._load (node:internal/modules/cjs/loader:822:12)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:81:12)
    at node:internal/main/run_main_module:17:47
