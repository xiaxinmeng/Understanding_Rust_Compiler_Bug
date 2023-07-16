
thread 'rustc' panicked at 'Failed to get crate data for crate18', compiler/rustc_metadata/src/creader.rs:136:32
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f82664191d0e8764b7435b9d72eb0e366b8b1464/library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at /rustc/f82664191d0e8764b7435b9d72eb0e366b8b1464/library/std/src/panicking.rs:435:5
   2: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
   3: rustc_metadata::creader::CrateLoader::maybe_process_path_extern
   4: rustc_resolve::Resolver::extern_prelude_get
   5: rustc_resolve::macros::<impl rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope
   6: rustc_resolve::Resolver::resolve_path_with_ribs::{{closure}}
   7: rustc_resolve::Resolver::resolve_path_with_ribs
   8: rustc_resolve::imports::ImportResolver::resolve_imports
   9: rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::resolve_imports
  10: rustc_expand::expand::MacroExpander::fully_expand_fragment
  11: rustc_expand::expand::MacroExpander::expand_crate
  12: rustc_session::utils::<impl rustc_session::session::Session>::time
  13: rustc_interface::passes::configure_and_expand_inner
  14: rustc_interface::passes::configure_and_expand::{{closure}}
  15: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  16: rustc_interface::passes::configure_and_expand
  17: rustc_interface::queries::Queries::expansion
  18: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  19: rustc_span::with_source_map
  20: rustc_interface::interface::create_compiler_and_run
  21: scoped_tls::ScopedKey<T>::set
