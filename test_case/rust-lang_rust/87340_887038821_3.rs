
error: at least one trait must be specified
 --> mutant.rs:1:28
  |
1 | type PartiallyDefined<T> = impl 'static;
  |                            ^^^^^^^^^^^^

error[E0658]: `impl Trait` in type aliases is unstable
 --> mutant.rs:1:28
  |
1 | type PartiallyDefined<T> = impl 'static;
  |                            ^^^^^^^^^^^^
  |
  = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
  = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: UnresolvedTy(_#0t)', compiler/rustc_typeck/src/check/writeback.rs:499:75
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9c25eb7aa3a71fb951564b0ddf131be59c2c951d/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/9c25eb7aa3a71fb951564b0ddf131be59c2c951d/library/core/src/panicking.rs:92:14
   2: core::result::unwrap_failed
             at /rustc/9c25eb7aa3a71fb951564b0ddf131be59c2c951d/library/core/src/result.rs:1599:5
   3: rustc_typeck::check::writeback::WritebackCx::visit_opaque_types
   4: rustc_typeck::check::writeback::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_type_vars_in_body
   5: rustc_infer::infer::InferCtxtBuilder::enter
   6: rustc_typeck::check::typeck
   7: rustc_query_system::query::plumbing::get_query_impl
   8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
   9: rustc_typeck::collect::fn_sig
  10: rustc_query_system::query::plumbing::get_query_impl
  11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::fn_sig
  12: rustc_typeck::collect::convert_item
  13: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  14: rustc_middle::hir::map::Map::visit_item_likes_in_module
  15: rustc_typeck::collect::collect_mod_item_types
  16: rustc_query_system::query::plumbing::get_query_impl
  17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_mod_item_types
  18: rustc_session::session::Session::track_errors
  19: rustc_typeck::check_crate
  20: rustc_interface::passes::analysis
  21: rustc_query_system::query::plumbing::get_query_impl
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  23: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  24: rustc_span::with_source_map
  25: rustc_interface::interface::create_compiler_and_run
  26: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (9c25eb7aa 2021-07-25) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type staticlib

query stack during panic:
#0 [typeck] type-checking `partially_defined`
#1 [fn_sig] computing function signature of `partially_defined`
#2 [collect_mod_item_types] collecting item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
