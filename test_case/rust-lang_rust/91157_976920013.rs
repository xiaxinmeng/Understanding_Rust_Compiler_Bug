plain
    |
note: function defined here
   --> /checkout/src/tools/clippy/clippy_lints/src/lib.rs:414:8
    |
414 | pub fn register_pre_expansion_lints(store: &mut rustc_lint::LintStore, sess: &Session, conf: &Conf) {

For more information about this error, try `rustc --explain E0061`.
[RUSTC-TIMING] rls_rustc test:false 0.447
error: could not compile `rls-rustc` due to previous error
---
test [compile-fail] compile-fail/modifying_constants.rs ... ok

error: failure produced the wrong error: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/write_bytes_overflow.rs" "-L" "/tmp/compiletestfeWSAL" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestfeWSAL/intrinsics/write_bytes_overflow.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestfeWSAL/intrinsics/write_bytes_overflow.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Size::mul: 4 * 9223372036854775808 doesn't fit in u64', /checkout/compiler/rustc_target/src/abi/mod.rs:449:21
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   1: core::panicking::panic_fmt
   2: <rustc_target::abi::Size as core::ops::arith::Mul<u64>>::mul
   3: <rustc_const_eval::interpret::eval_context::InterpCx<miri::machine::Evaluator>>::write_bytes_intrinsic
   4: <rustc_const_eval::interpret::eval_context::InterpCx<miri::machine::Evaluator>>::emulate_intrinsic
   5: <rustc_const_eval::interpret::eval_context::InterpCx<miri::machine::Evaluator> as miri::shims::intrinsics::EvalContextExt>::call_intrinsic
   6: <rustc_const_eval::interpret::eval_context::InterpCx<miri::machine::Evaluator>>::eval_fn_call
   7: <rustc_const_eval::interpret::eval_context::InterpCx<miri::machine::Evaluator>>::terminator
   8: miri::eval::eval_entry
   9: <rustc_interface::passes::QueryContext>::enter::<<miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis::{closure#0}, ()>
  10: <miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis
  11: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  12: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  13: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  14: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 1 test
test [ui] ui-toml/max_suggested_slice_pattern_length/index_refutable_slice.rs ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 1 test
---
.......... (50/53)
..        (53/53)


/checkout/src/test/rustdoc-gui/search-tab-selection-if-current-is-empty.goml search-tab-selection-if-current-is-empty... FAILED
[ERROR] (line 4) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `// Waiting for the search results to appear...
wait-for: "#titles"`



command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:45
