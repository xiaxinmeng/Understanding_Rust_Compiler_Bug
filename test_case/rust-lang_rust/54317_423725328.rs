
[00:59:02] ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
[00:59:02] error: test run failed!
[00:59:02] status: exit code: 101
[00:59:02] status: exit code: 101
[00:59:02] command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a.wasm"
[00:59:02] ------------------------------------------
[00:59:02] 
[00:59:02] ------------------------------------------
[00:59:02] stderr:
[00:59:02] stderr:
[00:59:02] ------------------------------------------
[00:59:02] RuntimeError: unreachable
[00:59:02]     at __rust_start_panic (wasm-function[148]:1)
[00:59:02]     at rust_panic.llvm.7229658430215002035 (wasm-function[142]:38)
[00:59:02]     at std::panicking::rust_panic_with_hook::h226a58a5977f2e17 (wasm-function[137]:500)
[00:59:02]     at std::panicking::continue_panic_fmt::h5f0126bd5d9da93f (wasm-function[136]:155)
[00:59:02]     at rust_begin_unwind (wasm-function[135]:3)
[00:59:02]     at core::panicking::panic_fmt::hebc42c0af7746402 (wasm-function[245]:80)
[00:59:02]     at core::result::unwrap_failed::hb8e1b756b0aad0f7 (wasm-function[19]:149)
[00:59:02]     at _$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$::unwrap::h047f9d6689dc96bf (wasm-function[1]:43)
[00:59:02]     at dbg_macro_expected_behavior::main::h80c10857552d47ce (wasm-function[6]:2332)
[00:59:02]     at std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hffb7cd8ec38eb3e9 (wasm-function[9]:25)
[00:59:02]     at std::sys_common::backtrace::__rust_begin_short_backtrace::h178ce1cdbcfd3352 (wasm-function[68]:8)
[00:59:02]     at std::panicking::try::do_call::h92ed3f8ab1f1fe4c (.llvm.7229658430215002035) (wasm-function[134]:20)
[00:59:02]     at __rust_maybe_catch_panic (wasm-function[147]:5)
[00:59:02]     at std::rt::lang_start_internal::had2eaac6c6c88dfd (wasm-function[69]:150)
[00:59:02]     at std::rt::lang_start::hecee5b62d0f0e846 (wasm-function[8]:42)
[00:59:02]     at main (wasm-function[7]:11)
[00:59:02]     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:136:20)
[00:59:02]     at Module._compile (module.js:641:30)
[00:59:02]     at Object.Module._extensions..js (module.js:652:10)
[00:59:02]     at Module.load (module.js:560:32)
[00:59:02] ------------------------------------------
[00:59:02] 
[00:59:02] 
[00:59:02] thread '[ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
