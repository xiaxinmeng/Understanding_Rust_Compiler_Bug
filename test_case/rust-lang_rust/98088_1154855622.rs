plain
test [ui] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test/ui/rfc-2011-nicer-assert-messages/assert-without-captures-does-not-create-unnecessary-code.rs stdout ----
error: test run failed!
status: exit status: 101
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2011-nicer-assert-messages/assert-without-captures-does-not-create-unnecessary-code/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[69]:0x447b)
    at rust_panic (<anonymous>:wasm-function[67]:0x4467)
    at _ZN3std9panicking20rust_panic_with_hook17h2290fdf1f47a7204E (<anonymous>:wasm-function[66]:0x4437)
    at _ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h83100bd4fad592acE (<anonymous>:wasm-function[2]:0x239)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hc16c211cb2fcc7e2E (<anonymous>:wasm-function[1]:0x1fe)
    at _ZN3std9panicking11begin_panic17hdab6ce0f3d665554E (<anonymous>:wasm-function[5]:0x2bf)
    at _ZN56assert_without_captures_does_not_create_unnecessary_code4main17he508c3fb987653dcE (<anonymous>:wasm-function[10]:0x3a3)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h1c2b04e0f4e8fd60E (<anonymous>:wasm-function[3]:0x245)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h697794cb16f33288E (<anonymous>:wasm-function[4]:0x267)
    at _ZN3std2rt19lang_start_internal17hf6826b7f3fcb1bdfE (<anonymous>:wasm-function[48]:0x33e3)
    at main (<anonymous>:wasm-function[11]:0x3da)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47


---- [ui] src/test/ui/rfc-2011-nicer-assert-messages/all-not-available-cases.rs stdout ----


error: test run failed!
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2011-nicer-assert-messages/all-not-available-cases/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[63]:0x4392)
    at rust_panic (<anonymous>:wasm-function[61]:0x437e)
    at _ZN3std9panicking20rust_panic_with_hook17h2290fdf1f47a7204E (<anonymous>:wasm-function[60]:0x434e)
    at _ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h8acfbcdd09f22373E (<anonymous>:wasm-function[49]:0x3a6a)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h9a85531cfc71f79fE (<anonymous>:wasm-function[48]:0x39a9)
    at rust_begin_unwind (<anonymous>:wasm-function[55]:0x3fe5)
    at _ZN4core9panicking9panic_fmt17h107e9559ed8dc709E (<anonymous>:wasm-function[119]:0x5286)
    at _ZN23all_not_available_cases4main17hf48e2ea726d9473fE (<anonymous>:wasm-function[5]:0x2bf)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h9ebc33799ddf5a20E (<anonymous>:wasm-function[1]:0x1ce)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9099e1e34fee358E (<anonymous>:wasm-function[2]:0x1f0)
    at _ZN3std2rt19lang_start_internal17hf6826b7f3fcb1bdfE (<anonymous>:wasm-function[43]:0x32ff)
    at main (<anonymous>:wasm-function[6]:0x2f6)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47



---- [ui] src/test/ui/rfc-2011-nicer-assert-messages/all-expr-kinds.rs stdout ----
error: test run failed!
status: exit status: 101
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2011-nicer-assert-messages/all-expr-kinds/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[63]:0x43e3)
    at rust_panic (<anonymous>:wasm-function[61]:0x43cf)
    at _ZN3std9panicking20rust_panic_with_hook17h2290fdf1f47a7204E (<anonymous>:wasm-function[60]:0x439f)
    at _ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h8acfbcdd09f22373E (<anonymous>:wasm-function[49]:0x3abb)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h9a85531cfc71f79fE (<anonymous>:wasm-function[48]:0x39fa)
    at rust_begin_unwind (<anonymous>:wasm-function[55]:0x4036)
    at _ZN4core9panicking9panic_fmt17h107e9559ed8dc709E (<anonymous>:wasm-function[119]:0x52d7)
    at _ZN14all_expr_kinds4main17h5fddb57efaeffef3E (<anonymous>:wasm-function[5]:0x310)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he43830b7fc38b6e9E (<anonymous>:wasm-function[1]:0x21b)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8a51fceee2737975E (<anonymous>:wasm-function[2]:0x23d)
    at _ZN3std2rt19lang_start_internal17hf6826b7f3fcb1bdfE (<anonymous>:wasm-function[43]:0x3350)
    at main (<anonymous>:wasm-function[6]:0x347)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47



failures:
