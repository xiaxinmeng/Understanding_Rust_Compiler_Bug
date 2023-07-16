bash
error: traits in `#[derive(...)]` don't accept arguments
 --> src/main.rs:6:19
  |
6 |     #[derive(parse(from_os_str))]
  |                   ^^^^^^^^^^^^^ help: remove the arguments

thread 'rustc' panicked at '`derive` attribute should exist', src/librustc_expand/expand.rs:507:13
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1053
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1428
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  12: rust_begin_unwind
             at src/libstd/panicking.rs:378
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  14: core::option::expect_failed
             at src/libcore/option.rs:1203
  15: rustc_expand::expand::MacroExpander::fully_expand_fragment
  16: rustc_expand::expand::MacroExpander::expand_crate
  17: rustc_session::utils::<impl rustc_session::session::Session>::time
  18: rustc_interface::passes::configure_and_expand_inner
  19: rustc_interface::passes::configure_and_expand::{{closure}}
  20: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  21: rustc_interface::passes::configure_and_expand
  22: rustc_interface::queries::Queries::expansion
  23: rustc_interface::interface::run_compiler_in_existing_thread_pool
  24: syntax::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (abc3073c9 2020-02-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: aborting due to previous error

error: could not compile `bisectit`.
