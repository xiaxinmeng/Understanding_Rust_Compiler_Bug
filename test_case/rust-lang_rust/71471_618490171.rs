
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
             at src/libstd/io/mod.rs:1504
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
             at src/libstd/panicking.rs:515
  12: rust_begin_unwind
             at src/libstd/panicking.rs:419
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:111
  14: core::str::slice_error_fail
             at src/libcore/str/mod.rs:0
  15: core::str::traits::<impl core::slice::SliceIndex<str> for core::ops::range::Range<usize>>::index::{{closure}}
  16: rustc_parse::lexer::StringReader::next_token
  17: rustc_parse::lexer::tokentrees::TokenTreesReader::parse_all_token_trees
  18: rustc_parse::lexer::tokentrees::<impl rustc_parse::lexer::StringReader>::into_token_trees
  19: rustc_parse::maybe_file_to_stream
  20: rustc_parse::maybe_source_file_to_parser
  21: rustc_parse::new_parser_from_file
  22: rustc_parse::parse_crate_from_file
  23: rustc_session::utils::<impl rustc_session::session::Session>::time
  24: rustc_interface::passes::parse
  25: rustc_interface::queries::Queries::parse
  26: rustc_interface::interface::run_compiler_in_existing_thread_pool
  27: scoped_tls::ScopedKey<T>::set
  28: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
