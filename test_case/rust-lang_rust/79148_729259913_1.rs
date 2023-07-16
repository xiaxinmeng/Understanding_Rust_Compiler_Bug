
thread 'rustc' panicked at '`enum` keyword should exist in snippet', compiler/rustc_resolve/src/imports.rs:1443:26
stack backtrace:
   0: rust_begin_unwind
             at /rustc/cf9cf7c923eb01146971429044f216a3ca905e06/library/std/src/panicking.rs:495:5
   1: core::panicking::panic_fmt
             at /rustc/cf9cf7c923eb01146971429044f216a3ca905e06/library/core/src/panicking.rs:92:14
   2: core::option::expect_failed
             at /rustc/cf9cf7c923eb01146971429044f216a3ca905e06/library/core/src/option.rs:1260:5
   3: rustc_resolve::ModuleData::for_each_child
   4: rustc_resolve::imports::ImportResolver::finalize_imports
   5: rustc_resolve::Resolver::resolve_crate
   6: rustc_interface::passes::configure_and_expand_inner
   7: rustc_interface::passes::configure_and_expand::{{closure}}
   8: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
   9: rustc_interface::passes::configure_and_expand
  10: rustc_interface::queries::Queries::expansion
  11: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  12: rustc_span::with_source_map
  13: rustc_interface::interface::create_compiler_and_run
  14: rustc_span::with_session_globals
