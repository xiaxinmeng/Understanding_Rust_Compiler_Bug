`
    Checking dtoa v0.4.5 (/tmp/dtoa)
error: internal compiler error: src/librustc_middle/mir/interpret/mod.rs:389: expected memory, got Static(DefId(0:37 ~ dtoa[2f84]::DEC_DIGITS_LUT[0]))

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:490
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc_middle::ty::context::tls::with_opt::{{closure}}
  17: rustc_middle::ty::context::tls::with_opt
  18: rustc_middle::util::bug::opt_span_bug_fmt
  19: rustc_middle::util::bug::bug_fmt
  20: rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_const
  21: rustc_middle::mir::pretty_print_const
  22: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
  23: core::fmt::Formatter::write_fmt
             at src/libcore/fmt/mod.rs:1498
  24: <&T as core::fmt::Debug>::fmt
  25: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
  26: core::fmt::Formatter::write_fmt
             at src/libcore/fmt/mod.rs:1498
  27: <rustc_middle::mir::Operand as core::fmt::Debug>::fmt
  28: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
  29: core::fmt::Formatter::write_fmt
             at src/libcore/fmt/mod.rs:1498
  30: <rustc_middle::mir::Rvalue as core::fmt::Debug>::fmt
  31: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
  32: core::fmt::Formatter::write_fmt
             at src/libcore/fmt/mod.rs:1498
  33: <rustc_middle::mir::Statement as core::fmt::Debug>::fmt
  34: core::fmt::run
             at src/libcore/fmt/mod.rs:1103
  35: core::fmt::write
             at src/libcore/fmt/mod.rs:1078
  36: core::fmt::Write::write_fmt
             at /rustc/8970e8bcf6153d1ead2283f1a0ed7b192230eca6/src/libcore/fmt/mod.rs:193
  37: alloc::fmt::format
             at src/liballoc/fmt.rs:586
  38: rustc_mir::util::pretty::write_mir_pretty
  39: rustc_mir::transform::dump_mir::emit_mir
  40: rustc_interface::passes::start_codegen
  41: rustc_middle::ty::context::tls::enter_global
  42: rustc_interface::queries::Queries::ongoing_codegen
  43: rustc_interface::interface::run_compiler_in_existing_thread_pool
  44: scoped_tls::ScopedKey<T>::set
  45: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.45.0-nightly (8970e8bcf 2020-05-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: aborting due to previous error

error: could not compile `dtoa`.

To learn more, run the command again with --verbose.
