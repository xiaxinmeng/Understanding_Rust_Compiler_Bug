
thread 'rustc' panicked at 'non-eager expansion without a parent scope', compiler/rustc_resolve/src/macros.rs:238:22
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::option::expect_failed
   3: rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::resolve_macro_invocation
   4: rustc_expand::expand::MacroExpander::fully_expand_fragment
   5: rustc_expand::expand::MacroExpander::expand_crate
   6: rustc_session::utils::<impl rustc_session::session::Session>::time
   7: rustc_interface::passes::configure_and_expand_inner
   8: rustc_interface::passes::configure_and_expand::{{closure}}
   9: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  10: rustc_interface::passes::configure_and_expand
  11: rustc_interface::queries::Queries::expansion
  12: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  13: rustc_span::with_source_map
  14: rustc_interface::interface::create_compiler_and_run
