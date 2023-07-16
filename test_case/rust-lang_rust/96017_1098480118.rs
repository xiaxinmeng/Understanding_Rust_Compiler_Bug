
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "Provided allocation has wrong size for slot count 131072"', compiler/rustc_metadata/src/rmeta/decoder.rs:263:29
stack backtrace:
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "Provided allocation has wrong size for slot count 131072"', compiler/rustc_metadata/src/rmeta/decoder.rs:263:29
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "Provided allocation has wrong size for slot count 131072"', compiler/rustc_metadata/src/rmeta/decoder.rs:263:29
stack backtrace:
stack backtrace:
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "Provided allocation has wrong size for slot count 131072"', compiler/rustc_metadata/src/rmeta/decoder.rs:263:29
stack backtrace:
   0: rust_begin_unwind
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/std/src/panicking.rs:517:5
   0: rust_begin_unwind
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
   1: core::panicking::panic_fmt
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/core/src/panicking.rs:103:14
   2: core::result::unwrap_failed
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/core/src/result.rs:1617:5
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/core/src/panicking.rs:103:14
   3: rustc_metadata::rmeta::decoder::CrateMetadata::new
   4: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
   5: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
   2: core::result::unwrap_failed
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/core/src/result.rs:1617:5
   6: rustc_metadata::creader::CrateLoader::process_extern_crate
   7: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
   8: rustc_ast::visit::walk_item
   3: rustc_metadata::rmeta::decoder::CrateMetadata::new
   4: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
   9: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
  10: rustc_expand::expand::AstFragment::visit_with
  11: rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::visit_ast_fragment_with_placeholders
   5: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
  12: rustc_expand::expand::MacroExpander::collect_invocations
  13: rustc_expand::expand::MacroExpander::fully_expand_fragment
  14: rustc_expand::expand::MacroExpander::expand_crate
  15: rustc_session::utils::<impl rustc_session::session::Session>::time
  16: rustc_interface::passes::configure_and_expand
   6: rustc_metadata::creader::CrateLoader::process_extern_crate
  17: rustc_interface::queries::Queries::expansion
  18: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  19: rustc_span::with_source_map
  20: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

   7: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (aa8f2d432 2021-09-18) running on aarch64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debug-assertions=off --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
   8: rustc_ast::visit::walk_item
   9: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
  10: rustc_expand::expand::AstFragment::visit_with
  11: rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::visit_ast_fragment_with_placeholders
  12: rustc_expand::expand::MacroExpander::collect_invocations
  13: rustc_expand::expand::MacroExpander::fully_expand_fragment
  14: rustc_expand::expand::MacroExpander::expand_crate
  15: rustc_session::utils::<impl rustc_session::session::Session>::time
  16: rustc_interface::passes::configure_and_expand
  17: rustc_interface::queries::Queries::expansion
  18: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  19: rustc_span::with_source_map
  20: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (aa8f2d432 2021-09-18) running on aarch64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debug-assertions=off --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
   0: rust_begin_unwind
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/core/src/panicking.rs:103:14
   2: core::result::unwrap_failed
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/core/src/result.rs:1617:5
error: could not compile `typenum`
warning: build failed, waiting for other jobs to finish...
   3: rustc_metadata::rmeta::decoder::CrateMetadata::new
   4: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
   5: rustc_metadata::creader::CrateLoader::process_extern_crate
   6: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
   7: rustc_ast::visit::walk_item
   8: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
   9: rustc_expand::expand::AstFragment::visit_with
  10: rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::visit_ast_fragment_with_placeholders
  11: rustc_expand::expand::MacroExpander::collect_invocations
  12: rustc_expand::expand::MacroExpander::fully_expand_fragment
  13: rustc_expand::expand::MacroExpander::expand_crate
  14: rustc_session::utils::<impl rustc_session::session::Session>::time
  15: rustc_interface::passes::configure_and_expand
  16: rustc_interface::queries::Queries::expansion
  17: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  18: rustc_span::with_source_map
  19: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (aa8f2d432 2021-09-18) running on aarch64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
   0: rust_begin_unwind
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/core/src/panicking.rs:103:14
   2: core::result::unwrap_failed
             at /rustc/aa8f2d432b23575929a48f87b8746f41ba723318/library/core/src/result.rs:1617:5
   3: rustc_metadata::rmeta::decoder::CrateMetadata::new
   4: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
   5: rustc_metadata::creader::CrateLoader::maybe_resolve_crate
   6: rustc_metadata::creader::CrateLoader::process_extern_crate
   7: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
   8: rustc_ast::visit::walk_item
   9: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item
  10: rustc_expand::expand::AstFragment::visit_with
  11: rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::visit_ast_fragment_with_placeholders
  12: rustc_expand::expand::MacroExpander::collect_invocations
  13: rustc_expand::expand::MacroExpander::fully_expand_fragment
  14: rustc_expand::expand::MacroExpander::expand_crate
  15: rustc_session::utils::<impl rustc_session::session::Session>::time
  16: rustc_interface::passes::configure_and_expand
  17: rustc_interface::queries::Queries::expansion
  18: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  19: rustc_span::with_source_map
  20: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (aa8f2d432 2021-09-18) running on aarch64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debug-assertions=off --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: build failed

