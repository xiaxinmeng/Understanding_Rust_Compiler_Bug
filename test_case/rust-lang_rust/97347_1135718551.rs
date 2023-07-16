plain
test [ui] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test/ui/drop/repeat-drop.rs stdout ----
error: test run failed!
status: exit status: 101
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/repeat-drop/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[70]:0x4482)
    at rust_panic (<anonymous>:wasm-function[68]:0x446e)
    at _ZN3std9panicking20rust_panic_with_hook17h438412e025c69b41E (<anonymous>:wasm-function[67]:0x443e)
    at _ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h3fa2c125e85564d1E (<anonymous>:wasm-function[2]:0x23a)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h175251abdd52dbe6E (<anonymous>:wasm-function[1]:0x1ff)
    at _ZN3std9panicking11begin_panic17hcf6af3f636240d9fE (<anonymous>:wasm-function[5]:0x2c4)
    at _ZN4core3ptr43drop_in_place$LT$repeat_drop..DropPanic$GT$17h4c782ed515570b9bE (<anonymous>:wasm-function[8]:0x313)
    at _ZN11repeat_drop4main17h9a2d0884828a3022E (<anonymous>:wasm-function[11]:0x3aa)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h1a420364536eafdfE (<anonymous>:wasm-function[3]:0x246)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1a9ec836ff36213bE (<anonymous>:wasm-function[4]:0x268)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
    at _ZN3std2rt19lang_start_internal17h550cf429c961372bE (<anonymous>:wasm-function[49]:0x33ea)
    at main (<anonymous>:wasm-function[12]:0x3e1)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47



failures:
