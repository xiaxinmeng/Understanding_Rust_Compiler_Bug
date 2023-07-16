
thread 'rustc' panicked at 'compiler/rustc_resolve/src/imports.rs:1056:33: inconsistent resolution for an import', /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panic.rs:59:5
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   3: rustc_middle::ty::context::tls::with_opt::{{closure}}
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::span_bug_fmt
   7: rustc_resolve::imports::ImportResolver::finalize_import::{{closure}}
   8: rustc_resolve::imports::ImportResolver::finalize_import
   9: rustc_resolve::imports::ImportResolver::finalize_imports
  10: rustc_session::utils::<impl rustc_session::session::Session>::time
  11: rustc_resolve::Resolver::resolve_crate
  12: rustc_interface::passes::configure_and_expand_inner
  13: rustc_interface::passes::configure_and_expand::{{closure}}
  14: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  15: rustc_interface::passes::configure_and_expand
  16: rustc_interface::queries::Queries::expansion
  17: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  18: rustc_span::with_source_map
  19: rustc_interface::interface::create_compiler_and_run
  20: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: internal compiler error: unexpected panic
