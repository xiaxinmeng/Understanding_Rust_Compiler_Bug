plain
[RUSTC-TIMING] coretests test:true 36.022
    Finished release [optimized] target(s) in 40.52s
     Running tests/lib.rs (build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/coretests-d7406a103b3fd9c6.wasm)
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[1988]:0x1503ff)
    at rust_panic (<anonymous>:wasm-function[1977]:0x15010b)
    at _ZN3std9panicking20rust_panic_with_hook17h5ccb5bdcd34631a8E (<anonymous>:wasm-function[1970]:0x14fdd0)
    at _ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h4b079a85c853017cE (<anonymous>:wasm-function[718]:0xd50c3)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hebc5c9bf2558c4a9E (<anonymous>:wasm-function[717]:0xd508a)
    at _ZN3std9panicking11begin_panic17hc9d2eb3c14256968E (<anonymous>:wasm-function[173]:0x84a51)
    at _ZN9coretests4iter8adapters3zip45test_zip_trusted_random_access_next_back_drop17h78266124f4eb31b9E (<anonymous>:wasm-function[216]:0x891fb)
    at _ZN4core3ops8function6FnOnce9call_once17h226bf9d90c80c729E (<anonymous>:wasm-function[852]:0xddfee)
    at _ZN4test28__rust_begin_short_backtrace17h64f35c79219c786dE (<anonymous>:wasm-function[1600]:0x11c3cf)
    at _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2aad70032d941592E (<anonymous>:wasm-function[1599]:0x11c3b1)
    at _ZN4test8run_test14run_test_inner17h05c809612e14f86aE (<anonymous>:wasm-function[1728]:0x13a194)
    at _ZN4test8run_test17h5daeaa0299d1a7c2E (<anonymous>:wasm-function[1682]:0x12d701)
    at _ZN4test7console17run_tests_console17h42d68f80f0f18bf7E (<anonymous>:wasm-function[1676]:0x1295a2)
    at _ZN4test9test_main17h95710762cd3a72ccE (<anonymous>:wasm-function[1726]:0x139076)
    at _ZN4test16test_main_static17h0839790a395daff6E (<anonymous>:wasm-function[1727]:0x139efb)
    at _ZN9coretests4main17ha70dbf5e8e4935cdE (<anonymous>:wasm-function[1358]:0xfbaed)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h12ef3977288498daE (<anonymous>:wasm-function[719]:0xd50e2)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h98d8bcc2dfd3ba70E (<anonymous>:wasm-function[720]:0xd5116)
    at _ZN3std2rt19lang_start_internal17he10975b331fcd316E (<anonymous>:wasm-function[1978]:0x1501ca)
    at main (<anonymous>:wasm-function[1359]:0xfbb23)
error: test failed, to rerun pass '-p core --test coretests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-unknown" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/mir-opt src/test/codegen-units library/core
Build completed unsuccessfully in 0:21:17
