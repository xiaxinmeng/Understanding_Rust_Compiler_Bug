
[00:56:18] ---- [run-pass] run-pass/invalid_const_promotion.rs stdout ----
[00:56:18] 
[00:56:18] error: test run failed!
[00:56:18] status: exit code: 101
[00:56:18] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/invalid_const_promotion/a.wasm"
[00:56:18] stdout:
[00:56:18] ------------------------------------------
[00:56:18] 
[00:56:18] ------------------------------------------
[00:56:18] stderr:
[00:56:18] ------------------------------------------
[00:56:18] RuntimeError: unreachable
[00:56:18]     at __rust_start_panic (wasm-function[131]:1)
[00:56:18]     at rust_panic.llvm.9502800419846304078 (wasm-function[127]:30)
[00:56:18]     at std::panicking::rust_panic_with_hook::h0d5a137ad2df1a11 (wasm-function[122]:441)
[00:56:18]     at std::panicking::continue_panic_fmt::hf8718f064fd9e77b (wasm-function[120]:120)
[00:56:18]     at rust_begin_unwind (wasm-function[119]:3)
[00:56:18]     at core::panicking::panic_fmt::hb53c33b6d4981cfb (wasm-function[228]:70)
[00:56:18]     at core::result::unwrap_failed::h0ceaf4312c8cc139 (wasm-function[9]:130)
[00:56:18]     at _$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$::unwrap::h68dfea1c9fcddfb1 (wasm-function[10]:34)
[00:56:18]     at invalid_const_promotion::main::h0e4b68e8564f7e57 (wasm-function[12]:179)
[00:56:18]     at std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hceea268ccf57703c (wasm-function[5]:17)
[00:56:18]     at std::sys_common::backtrace::__rust_begin_short_backtrace::ha35b6b1b512e8e7b (wasm-function[57]:8)
[00:56:18]     at std::panicking::try::do_call::h6e356fa82a1037a1 (.llvm.9502800419846304078) (wasm-function[118]:20)
[00:56:18]     at __rust_maybe_catch_panic (wasm-function[130]:5)
[00:56:18]     at std::rt::lang_start_internal::h21c3cbc5f89250ba (wasm-function[58]:104)
[00:56:18]     at main (wasm-function[13]:33)
[00:56:18]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:136:20)
[00:56:18]     at Module._compile (module.js:641:30)
[00:56:18]     at Object.Module._extensions..js (module.js:652:10)
[00:56:18]     at Module.load (module.js:560:32)
[00:56:18]     at tryModuleLoad (module.js:503:12)
[00:56:18] 
[00:56:18] ------------------------------------------
[00:56:18] 
[00:56:18] thread '[run-pass] run-pass/invalid_const_promotion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:56:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:18] 
[00:56:18] 
[00:56:18] failures:
[00:56:18]     [run-pass] run-pass/invalid_const_promotion.rs
