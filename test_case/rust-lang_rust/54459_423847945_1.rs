
$ rustc +nightly foo.rs
error: internal compiler error: Error constructed but not emitted

thread 'main' panicked at 'explicit panic', librustc_errors/diagnostic_builder.rs:340:13
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:477
   5: std::panicking::begin_panic
   6: <rustc_errors::diagnostic_builder::DiagnosticBuilder<'a> as core::ops::drop::Drop>::drop
   7: syntax::parse::parser::Parser::parse_pat_fields
   8: syntax::parse::parser::Parser::parse_pat_with_range_pat
   9: syntax::parse::parser::Parser::parse_top_level_pat
  10: syntax::parse::parser::Parser::parse_pats
  11: syntax::parse::parser::Parser::parse_arm
  12: syntax::parse::parser::Parser::parse_bottom_expr
  13: syntax::parse::parser::Parser::parse_dot_or_call_expr
  14: syntax::parse::parser::Parser::parse_prefix_expr
  15: syntax::parse::parser::Parser::parse_assoc_expr_with
  16: syntax::parse::parser::Parser::parse_stmt_without_recovery
  17: syntax::parse::parser::Parser::parse_full_stmt
  18: syntax::parse::parser::Parser::parse_block_tail
  19: syntax::parse::parser::Parser::parse_inner_attrs_and_block
  20: syntax::parse::parser::Parser::parse_for_expr
  21: syntax::parse::parser::Parser::parse_bottom_expr
  22: syntax::parse::parser::Parser::parse_dot_or_call_expr
  23: syntax::parse::parser::Parser::parse_prefix_expr
  24: syntax::parse::parser::Parser::parse_assoc_expr_with
  25: syntax::parse::parser::Parser::parse_stmt_without_recovery
  26: syntax::parse::parser::Parser::parse_full_stmt
  27: syntax::parse::parser::Parser::parse_block_tail
  28: syntax::parse::parser::Parser::parse_inner_attrs_and_block
  29: syntax::parse::parser::Parser::parse_item_fn
  30: syntax::parse::parser::Parser::parse_item_implementation
  31: syntax::parse::parser::Parser::parse_item_
  32: syntax::parse::parser::Parser::parse_item
  33: syntax::parse::parser::Parser::parse_mod_items
  34: syntax::parse::parser::Parser::parse_crate_mod
  35: syntax::parse::parse_crate_from_file
  36: rustc_driver::driver::phase_1_parse_input::{{closure}}
  37: rustc::util::common::time
  38: rustc_driver::driver::phase_1_parse_input
  39: rustc_driver::driver::compile_input
  40: rustc_driver::run_compiler_with_pool
  41: <scoped_tls::ScopedKey<T>>::set
  42: rustc_driver::run_compiler
  43: <scoped_tls::ScopedKey<T>>::set
  44: syntax::with_globals
  45: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  46: rustc_driver::run
  47: rustc_driver::main
  48: std::rt::lang_start::{{closure}}
  49: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  50: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  51: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  52: main
  53: __libc_start_main
  54: <unknown>
error: aborting due to previous error


error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.30.0-nightly (4591a245c 2018-09-22) running on x86_64-unknown-linux-gnu
