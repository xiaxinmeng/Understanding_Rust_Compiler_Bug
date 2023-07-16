
thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_parse/parser/mod.rs:446:22
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:188
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:205
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
  12: std::panicking::begin_panic
  13: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_closure_expr
  14: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_bottom_expr
  15: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_dot_or_call_expr
  16: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_prefix_expr
  17: rustc_parse::parser::expr::<impl rustc_parse::parser::Parser>::parse_assoc_expr_with
  18: rustc_parse::parser::stmt::<impl rustc_parse::parser::Parser>::parse_stmt_without_recovery
  19: rustc_parse::parser::stmt::<impl rustc_parse::parser::Parser>::parse_full_stmt
  20: rustc_parse::parser::stmt::<impl rustc_parse::parser::Parser>::parse_block_tail
  21: rustc_parse::parser::stmt::<impl rustc_parse::parser::Parser>::parse_inner_attrs_and_block
  22: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_item_fn
  23: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_item_implementation
  24: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_item_
  25: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_item
  26: rustc_parse::parser::module::<impl rustc_parse::parser::Parser>::parse_mod_items
  27: rustc_parse::parser::module::<impl rustc_parse::parser::Parser>::parse_crate_mod
  28: rustc_parse::parse_crate_from_file
  29: rustc_interface::passes::parse::{{closure}}
  30: rustc_interface::passes::parse
  31: rustc_interface::queries::Query<T>::compute
  32: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::parse
  33: rustc_interface::interface::run_compiler_in_existing_thread_pool
  34: std::thread::local::LocalKey<T>::with
  35: scoped_tls::ScopedKey<T>::set
  36: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (4f03f4a98 2019-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: aborting due to 2 previous errors
