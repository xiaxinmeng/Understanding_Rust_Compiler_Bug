
thread 'rustc' panicked at 'src/librustc_resolve/imports.rs:906: inconsistent resolution for an import', /rustc/1836e3b42a5b2f37fd79104eedbe8f48a5afdee6/src/libstd/macros.rs:13:23
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic
   8: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc_middle::ty::context::tls::with_opt::{{closure}}
  10: rustc_middle::ty::context::tls::with_opt
  11: rustc_middle::util::bug::opt_span_bug_fmt
  12: rustc_middle::util::bug::span_bug_fmt
  13: rustc_resolve::imports::ImportResolver::finalize_import
  14: rustc_resolve::imports::ImportResolver::finalize_imports
  15: rustc_resolve::Resolver::resolve_crate
  16: rustc_interface::passes::configure_and_expand_inner
  17: rustc_interface::passes::configure_and_expand::{{closure}}
  18: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  19: rustc_interface::passes::configure_and_expand
  20: rustc_interface::queries::Queries::expansion
  21: rustc_interface::interface::run_compiler_in_existing_thread_pool
  22: scoped_tls::ScopedKey<T>::set
  23: rustc_ast::attr::with_globals
