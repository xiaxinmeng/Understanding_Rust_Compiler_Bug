
> RUST_BACKTRACE=1 cargo check --tests
    Checking ice v0.1.0 (~/ice)
thread 'rustc' panicked at 'Failed to get crate data for crate18', compiler/rustc_metadata/src/creader.rs:136:32
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:435:5
   2: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
   3: rustc_metadata::creader::CrateLoader::process_path_extern
   4: rustc_resolve::Resolver::extern_prelude_get
   5: rustc_resolve::macros::<impl rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope
   6: rustc_resolve::Resolver::resolve_path_with_ribs::{{closure}}
   7: rustc_resolve::Resolver::resolve_path_with_ribs
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

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.1 (9bc8c42bb 2021-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `ice`

To learn more, run the command again with --verbose.
