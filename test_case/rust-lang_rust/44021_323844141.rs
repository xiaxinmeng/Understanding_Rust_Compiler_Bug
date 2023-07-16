bash
$ RUST_BACKTRACE=1 rustc-nightly tmp.rs
error: expected expression, found `}`
 --> tmp.rs:4:1
  |
4 | }
  | ^

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-nightly (8c303ed87 2017-08-20) running on i686-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 1', /checkout/src/liballoc/vec.rs:1564:14
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic_bounds_check
             at /checkout/src/libcore/panicking.rs:58
  10: syntax::parse::parser::Parser::parse_impl_item
  11: syntax::parse::parser::Parser::parse_item_impl
  12: syntax::parse::parser::Parser::parse_item_
  13: syntax::parse::parser::Parser::parse_item
  14: syntax::parse::parser::Parser::parse_mod_items
  15: syntax::parse::parser::Parser::parse_crate_mod
  16: syntax::parse::parse_crate_from_file
  17: rustc_driver::driver::phase_1_parse_input::{{closure}}
  18: rustc_driver::driver::phase_1_parse_input
  19: rustc_driver::driver::compile_input
  20: rustc_driver::run_compiler

$
