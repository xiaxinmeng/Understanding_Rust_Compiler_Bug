
     Running `rustc --crate-name get_stuff_done src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=57f9cdfec93028f0 -C extra-filename=-57f9cdfec93028f0 --out-dir /Users/reembodied/Documents/workplace/GSD/target/debug/deps -C incremental=/Users/reembodied/Documents/workplace/GSD/target/debug/incremental -L dependency=/Users/reembodied/Documents/workplace/GSD/target/debug/deps --extern r2d2=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/libr2d2-5ee46b683767f62a.rlib --extern dotenv=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/libdotenv-11d6c368f55e51dc.rlib --extern serde_json=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/libserde_json-4d2498fc1773ad64.rlib --extern serde=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/libserde-a870014ba779aecf.rlib --extern rocket_contrib=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/librocket_contrib-c91514945a0602e5.rlib --extern rocket=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/librocket-fa1e8f79ed259746.rlib --extern diesel=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/libdiesel-c5fe6a028fa90bc3.rlib --extern rocket_codegen=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/librocket_codegen-e22163b7f02193ea.dylib --extern chrono=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/libchrono-01e8ade0bcc6f635.rlib --extern serde_derive=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/libserde_derive-fe1669d5f547364d.dylib --extern r2d2_diesel=/Users/reembodied/Documents/workplace/GSD/target/debug/deps/libr2d2_diesel-3f90588d0f23df53.rlib -L native=/Users/reembodied/Documents/workplace/GSD/target/debug/build/ring-917da51119e12995/out`
error: internal compiler error: Error constructed but not emitted

thread 'rustc' panicked at 'explicit panic', librustc_errors/diagnostic_builder.rs:242:13
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: core::ops::function::Fn::call
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: <rustc_errors::diagnostic_builder::DiagnosticBuilder<'a> as core::ops::drop::Drop>::drop
   8: syntax::ext::expand::MacroExpander::expand_invoc
   9: syntax::ext::expand::MacroExpander::expand
  10: syntax::ext::expand::MacroExpander::expand_crate
  11: rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}
  12: rustc_driver::driver::phase_2_configure_and_expand_inner
  13: rustc_driver::driver::compile_input
14: rustc_driver::run_compiler
