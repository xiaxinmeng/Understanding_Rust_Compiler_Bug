
warning: trait objects without an explicit `dyn` are deprecated
 --> mutant.rs:1:25
  |
1 | fn test_ref(x: &u32) -> std::future::Future<Output = u32> + '_ {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn std::future::Future<Output = u32> + '_`
  |
  = note: `#[warn(bare_trait_objects)]` on by default
  = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
  = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>

error[E0601]: `main` function not found in crate `mutant`
 --> mutant.rs:1:1
  |
1 | / fn test_ref(x: &u32) -> std::future::Future<Output = u32> + '_ {}
2 | | fn u() {
3 | |     test_ref & u
4 | | }
  | |_^ consider adding a `main` function to `mutant.rs`

error[E0746]: return type cannot have an unboxed trait object
 --> mutant.rs:1:25
  |
1 | fn test_ref(x: &u32) -> std::future::Future<Output = u32> + '_ {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
help: use some type `T` that is `T: Sized` as the return type if all return paths have the same type
  |
1 | fn test_ref(x: &u32) -> T {}
  |                         ^
help: use `impl std::future::Future<Output = u32> + '_` as the return type if all return paths have the same type but you want to expose only the trait in the signature
  |
1 | fn test_ref(x: &u32) -> impl std::future::Future<Output = u32> + '_ {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use a boxed trait object if all return paths implement trait `std::future::Future<Output = u32> + '_`
  |
1 | fn test_ref(x: &u32) -> Box<dyn std::future::Future<Output = u32> + '_> {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', compiler/rustc_middle/src/ty/sty.rs:968:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9c25eb7aa3a71fb951564b0ddf131be59c2c951d/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/9c25eb7aa3a71fb951564b0ddf131be59c2c951d/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/9c25eb7aa3a71fb951564b0ddf131be59c2c951d/library/core/src/panicking.rs:50:5
   3: <rustc_middle::ty::sty::TraitRef as rustc_middle::ty::ToPolyTraitRef>::to_poly_trait_ref
   4: rustc_typeck::check::method::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::lookup_method_in_trait
   5: rustc_typeck::check::op::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::lookup_op_method
   6: rustc_typeck::check::op::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::add_type_neq_err_label
   7: rustc_typeck::check::op::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_overloaded_binop
   8: rustc_typeck::check::op::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_binop
   9: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  10: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  11: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  12: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
  13: rustc_typeck::check::check::check_fn
  14: rustc_infer::infer::InferCtxtBuilder::enter
  15: rustc_typeck::check::typeck
  16: rustc_query_system::query::plumbing::get_query_impl
  17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  18: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
  19: rustc_typeck::check::typeck_item_bodies
  20: rustc_query_system::query::plumbing::get_query_impl
  21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck_item_bodies
  22: rustc_session::utils::<impl rustc_session::session::Session>::time
  23: rustc_typeck::check_crate
  24: rustc_interface::passes::analysis
  25: rustc_query_system::query::plumbing::get_query_impl
  26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  27: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  28: rustc_span::with_source_map
  29: rustc_interface::interface::create_compiler_and_run
  30: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (9c25eb7aa 2021-07-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [typeck] type-checking `u`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0601, E0746.
For more information about an error, try `rustc --explain E0601`.
