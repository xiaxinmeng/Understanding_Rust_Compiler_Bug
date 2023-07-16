plain
test [ui] tests/ui/async-await/issues/issue-67893.rs ... ok
test [ui] tests/ui/async-await/multiple-lifetimes/hrtb.rs ... ok
test [ui] tests/ui/async-await/move-part-await-return-rest-tuple.rs ... ok
test [ui] tests/ui/async-await/multiple-lifetimes/ret-ref.rs ... ok
test [ui] tests/ui/async-await/multiple-lifetimes/member-constraints-min-choice-issue-63033.rs ... ok
test [ui] tests/ui/async-await/mutually-recursive-async-impl-trait-type.rs#drop_tracking ... ok
test [ui] tests/ui/async-await/mutually-recursive-async-impl-trait-type.rs#drop_tracking_mir ... ok
test [ui] tests/ui/async-await/nested-in-impl.rs ... ok
test [ui] tests/ui/async-await/no-async-const.rs ... ok
---
test [ui] tests/ui/nll/maybe-initialized-drop-with-uninitialized-fragments.rs ... ok
test [ui] tests/ui/nll/member-constraints/min-choice.rs ... ok
test [ui] tests/ui/nll/maybe-initialized-drop.rs ... ok
test [ui] tests/ui/nll/issue-57960.rs ... ok
test [ui] tests/ui/nll/member-constraints/min-choice-reject-ambiguous.rs ... ok
test [ui] tests/ui/nll/mir_check_cast_closure.rs ... ok
test [ui] tests/ui/nll/mir_check_cast_reify.rs ... ok
test [ui] tests/ui/nll/mir_check_cast_unsafe_fn.rs ... ok
test [ui] tests/ui/nll/member-constraints/nested-impl-trait-fail.rs ... ok
---
---- [ui] tests/ui/consts/const-eval/issue-91827-extern-types.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-91827-extern-types" && RUST_TEST_THREADS="16" "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-91827-extern-types/a.wasm"
stdout: none
--- stderr -------------------------------
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[53]:0x3d2e)
    at rust_panic (<anonymous>:wasm-function[49]:0x3bf6)
    at _ZN3std9panicking20rust_panic_with_hook17hdc2587369679a530E (<anonymous>:wasm-function[48]:0x3bc6)
    at _ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h0d4e0c9153aed5a0E (<anonymous>:wasm-function[38]:0x32bc)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h113db0e4489a8b83E (<anonymous>:wasm-function[37]:0x31e6)
    at rust_begin_unwind (<anonymous>:wasm-function[43]:0x381d)
    at _ZN4core9panicking9panic_fmt17hcce51f70ea30a952E (<anonymous>:wasm-function[100]:0x45cb)
    at _ZN4core9panicking19assert_failed_inner17h8b898611f8bae466E (<anonymous>:wasm-function[116]:0x5053)
    at _ZN4core9panicking13assert_failed17h3b91ff5265d27903E (<anonymous>:wasm-function[7]:0x41f)
    at _ZN24issue_91827_extern_types4main17h66dc0d1f7a8dbae7E (<anonymous>:wasm-function[8]:0x45c)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h44fcb5436bc621cdE (<anonymous>:wasm-function[0]:0x1b4)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfb493f38d7fe943fE (<anonymous>:wasm-function[1]:0x1d6)
    at _ZN3std2rt19lang_start_internal17h14be199975ac12b5E (<anonymous>:wasm-function[35]:0x30e9)
    at main (<anonymous>:wasm-function[9]:0x495)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown



failures:
