
$ rustc +5fd88ccbf4d20c70960c1b0e76fe82325e23a35e foo.rs
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
             at libstd/panicking.rs:411
   6: <rustc_errors::diagnostic_builder::DiagnosticBuilder<'a> as core::ops::drop::Drop>::drop
             at librustc_errors/diagnostic_builder.rs:340
   7: syntax::parse::parser::Parser::parse_pat_fields
             at libcore/ptr.rs:59
             at libsyntax/parse/parser.rs:3938
   8: syntax::parse::parser::Parser::parse_pat_with_range_pat
             at libsyntax/parse/parser.rs:4131
   9: syntax::parse::parser::Parser::parse_top_level_pat
             at libsyntax/parse/parser.rs:4011
             at libsyntax/parse/parser.rs:3978
  10: syntax::parse::parser::Parser::parse_pats
             at libsyntax/parse/parser.rs:3650
  11: syntax::parse::parser::Parser::parse_arm
             at libsyntax/parse/parser.rs:3544
  12: syntax::parse::parser::Parser::parse_bottom_expr
             at libsyntax/parse/parser.rs:3519
             at libsyntax/parse/parser.rs:2410
  13: syntax::parse::parser::Parser::parse_dot_or_call_expr
             at libsyntax/parse/parser.rs:2614
  14: syntax::parse::parser::Parser::parse_prefix_expr
             at libsyntax/parse/parser.rs:2957
  15: syntax::parse::parser::Parser::parse_assoc_expr_with
             at libsyntax/parse/parser.rs:2987
  16: syntax::parse::parser::Parser::parse_stmt_without_recovery
             at libsyntax/parse/parser.rs:2969
             at libsyntax/parse/parser.rs:3632
             at libsyntax/parse/parser.rs:3622
             at libsyntax/parse/parser.rs:3632
             at libsyntax/parse/parser.rs:4741
  17: syntax::parse::parser::Parser::parse_full_stmt
             at libsyntax/parse/parser.rs:4888
  18: syntax::parse::parser::Parser::parse_block_tail
             at libsyntax/parse/parser.rs:4855
  19: syntax::parse::parser::Parser::parse_inner_attrs_and_block
             at libsyntax/parse/parser.rs:4845
  20: syntax::parse::parser::Parser::parse_for_expr
             at libsyntax/parse/parser.rs:3426
  21: syntax::parse::parser::Parser::parse_bottom_expr
             at libsyntax/parse/parser.rs:2370
  22: syntax::parse::parser::Parser::parse_dot_or_call_expr
             at libsyntax/parse/parser.rs:2614
  23: syntax::parse::parser::Parser::parse_prefix_expr
             at libsyntax/parse/parser.rs:2957
  24: syntax::parse::parser::Parser::parse_assoc_expr_with
             at libsyntax/parse/parser.rs:2987
  25: syntax::parse::parser::Parser::parse_stmt_without_recovery
             at libsyntax/parse/parser.rs:2969
             at libsyntax/parse/parser.rs:3632
             at libsyntax/parse/parser.rs:3622
             at libsyntax/parse/parser.rs:3632
             at libsyntax/parse/parser.rs:4741
  26: syntax::parse::parser::Parser::parse_full_stmt
             at libsyntax/parse/parser.rs:4888
  27: syntax::parse::parser::Parser::parse_block_tail
             at libsyntax/parse/parser.rs:4855
  28: syntax::parse::parser::Parser::parse_inner_attrs_and_block
             at libsyntax/parse/parser.rs:4845
  29: syntax::parse::parser::Parser::parse_item_fn
             at libsyntax/parse/parser.rs:5560
  30: syntax::parse::parser::Parser::parse_item_implementation
             at libsyntax/parse/parser.rs:7100
  31: syntax::parse::parser::Parser::parse_item_
             at libsyntax/parse/parser.rs:6883
             at libsyntax/parse/parser.rs:7460
             at libsyntax/parse/parser.rs:6882
  32: syntax::parse::parser::Parser::parse_item
             at libsyntax/parse/parser.rs:7503
  33: syntax::parse::parser::Parser::parse_mod_items
             at libsyntax/parse/parser.rs:6250
  34: syntax::parse::parser::Parser::parse_crate_mod
             at libsyntax/parse/parser.rs:7586
  35: syntax::parse::parse_crate_from_file
             at libsyntax/parse/mod.rs:133
  36: rustc_driver::driver::phase_1_parse_input::{{closure}}
             at librustc_driver/driver.rs:683
  37: rustc::util::common::time
             at librustc/util/common.rs:163
             at librustc/util/common.rs:157
  38: rustc_driver::driver::phase_1_parse_input
             at librustc_driver/driver.rs:682
  39: rustc_driver::driver::compile_input
             at librustc_driver/driver.rs:158
  40: rustc_driver::run_compiler_with_pool
             at librustc_driver/lib.rs:562
  41: rustc_driver::driver::spawn_thread_pool
             at librustc_driver/lib.rs:484
             at librustc_driver/driver.rs:76
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at librustc_driver/driver.rs:75
  42: rustc_driver::run_compiler
             at librustc_driver/lib.rs:483
  43: <scoped_tls::ScopedKey<T>>::set
             at librustc_driver/lib.rs:1744
             at librustc_driver/lib.rs:189
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at libsyntax/lib.rs:107
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  44: syntax::with_globals
             at libsyntax/lib.rs:106
  45: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  46: rustc_driver::run
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at librustc_driver/lib.rs:1573
             at librustc_driver/lib.rs:1584
             at librustc_driver/lib.rs:1658
             at librustc_driver/lib.rs:187
  47: rustc_driver::main
             at librustc_driver/lib.rs:1737
  48: std::rt::lang_start::{{closure}}
             at libstd/rt.rs:74
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

note: rustc 1.30.0-nightly (5fd88ccbf 2018-09-22) running on x86_64-unknown-linux-gnu
