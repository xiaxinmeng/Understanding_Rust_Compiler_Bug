
$ rustc +nightly -Vv
rustc 1.20.0-nightly (f590a44ce 2017-06-27)
binary: rustc
commit-hash: f590a44ce61888c78b9044817d8b798db5cd2ffd
commit-date: 2017-06-27
host: x86_64-unknown-linux-gnu
release: 1.20.0-nightly
LLVM version: 4.0

$ RUST_BACKTRACE=1 rustc +nightly --target powerpc64-unknown-linux-gnu src/test/run-make/extern-fn-struct-passing-abi/test.rs
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0-nightly (f590a44ce 2017-06-27) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Indirect`,
 right: `Direct`', /checkout/src/librustc_trans/abi.rs:444
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:511
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:495
   7: rustc_trans::abi::ArgType::make_indirect
   8: rustc_trans::abi::FnType::adjust_for_abi
   9: rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_terminator
  10: rustc_trans::mir::trans_mir
  11: rustc_trans::trans_item::TransItem::define
  12: rustc_trans::base::trans_crate
  13: rustc_driver::driver::phase_4_translate_to_llvm
  14: rustc_driver::driver::compile_input::{{closure}}
  15: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  16: rustc_driver::driver::phase_3_run_analysis_passes
  17: rustc_driver::driver::compile_input
  18: rustc_driver::run_compiler
