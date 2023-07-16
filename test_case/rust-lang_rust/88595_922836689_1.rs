
   Compiling playground v0.0.1 (/playground)
error[E0658]: generic associated types are unstable
 --> src/main.rs:4:5
  |
4 | /     type Iter<'b>: Iterator<Item = &'b T>
5 | |     where
6 | |         T: 'b;
  | |______________^
  |
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0658]: where clauses on associated types are unstable
 --> src/main.rs:4:5
  |
4 | /     type Iter<'b>: Iterator<Item = &'b T>
5 | |     where
6 | |         T: 'b;
  | |______________^
  |
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0658]: `impl Trait` in type aliases is unstable
  --> src/main.rs:17:20
   |
17 |     = iter::FromFn<impl FnMut() -> Option<&'b T>>;
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

error[E0658]: generic associated types are unstable
  --> src/main.rs:14:5
   |
14 | /     type Iter<'b>
15 | |     where
16 | |         T: 'b,
17 | |     = iter::FromFn<impl FnMut() -> Option<&'b T>>;
   | |__________________________________________________^
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0658]: where clauses on associated types are unstable
  --> src/main.rs:14:5
   |
14 | /     type Iter<'b>
15 | |     where
16 | |         T: 'b,
17 | |     = iter::FromFn<impl FnMut() -> Option<&'b T>>;
   | |__________________________________________________^
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

thread 'rustc' panicked at 'assertion failed: self.is_free_or_static(r_a) && self.is_free_or_static(r_b)', compiler/rustc_infer/src/infer/free_regions.rs:77:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/panicking.rs:50:5
   3: <rustc_infer::infer::free_regions::FreeRegionMap as rustc_infer::infer::free_regions::FreeRegionRelations>::sub_free_regions
   4: <rustc_infer::infer::InferCtxt as rustc_trait_selection::opaque_types::InferCtxtExt>::constrain_opaque_types
   5: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
   6: rustc_typeck::check::regionck::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::regionck_fn
   7: rustc_infer::infer::InferCtxtBuilder::enter
   8: rustc_typeck::check::typeck
   9: rustc_query_system::query::plumbing::get_query_impl
  10: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  11: rustc_typeck::collect::type_of::find_opaque_ty_constraints::ConstraintLocator::check
  12: <rustc_typeck::collect::type_of::find_opaque_ty_constraints::ConstraintLocator as rustc_hir::intravisit::Visitor>::visit_impl_item
  13: rustc_hir::intravisit::Visitor::visit_nested_impl_item
  14: rustc_hir::intravisit::walk_impl_item_ref
  15: rustc_hir::intravisit::walk_item
  16: rustc_typeck::collect::type_of::type_of
  17: rustc_query_system::query::plumbing::get_query_impl
  18: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  19: rustc_typeck::check::check::check_item_type
  20: rustc_middle::hir::map::Map::visit_item_likes_in_module
  21: rustc_typeck::check::check::check_mod_item_types
  22: rustc_query_system::query::plumbing::get_query_impl
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_item_types
  24: rustc_session::utils::<impl rustc_session::session::Session>::time
  25: rustc_typeck::check_crate
  26: rustc_interface::passes::analysis
  27: rustc_query_system::query::plumbing::get_query_impl
  28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  29: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  30: rustc_span::with_source_map
  31: rustc_interface::interface::create_compiler_and_run
  32: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0 (c8dfcfe04 2021-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `<impl at src/main.rs:13:1: 22:2>::iter`
#1 [type_of] computing type of `<impl at src/main.rs:13:1: 22:2>::Iter::{opaque#0}`
#2 [check_mod_item_types] checking item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0658`.
error: could not compile `playground` due to 5 previous errors
