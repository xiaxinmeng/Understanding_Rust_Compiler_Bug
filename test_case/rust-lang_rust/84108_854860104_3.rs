
error: free static item without body
 --> mutant.rs:1:1
  |
1 | static FOO: (PartialEq<Item>, u32);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
  |                                   |
  |                                   help: provide a definition for the static: `= <expr>;`

error[E0412]: cannot find type `Item` in this scope
 --> mutant.rs:1:24
  |
1 | static FOO: (PartialEq<Item>, u32);
  |                        ^^^^ not found in this scope

warning: trait objects without an explicit `dyn` are deprecated
 --> mutant.rs:1:14
  |
1 | static FOO: (PartialEq<Item>, u32);
  |              ^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PartialEq<Item>`
  |
  = note: `#[warn(bare_trait_objects)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
  = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>

error[E0601]: `main` function not found in crate `mutant`
 --> mutant.rs:1:1
  |
1 | static FOO: (PartialEq<Item>, u32);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ consider adding a `main` function to `mutant.rs`

error: internal compiler error: compiler/rustc_middle/src/ty/layout.rs:370:17: univariant: field #2 of `(dyn PartialEq<[type error]>, u32)` comes after unsized field

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:1007:9
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::univariant_uninterned
   8: rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached
   9: rustc_middle::ty::layout::layout_raw
  10: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::layout_raw>::compute
  11: rustc_query_system::query::plumbing::get_query_impl
  12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_raw
  13: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  14: rustc_typeck::check::check::check_static_inhabited
  15: rustc_typeck::check::check::check_item_type
  16: rustc_middle::hir::map::Map::visit_item_likes_in_module
  17: rustc_typeck::check::check::check_mod_item_types
  18: rustc_query_system::query::plumbing::get_query_impl
  19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_item_types
  20: rustc_session::utils::<impl rustc_session::session::Session>::time
  21: rustc_typeck::check_crate
  22: rustc_interface::passes::analysis
  23: rustc_query_system::query::plumbing::get_query_impl
  24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  25: rustc_interface::passes::QueryContext::enter
  26: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (cc77ba46f 2021-06-03) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [layout_raw] computing layout of `(dyn std::cmp::PartialEq<[type error]>, u32)`
#1 [check_mod_item_types] checking item types in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 4 previous errors; 1 warning emitted

Some errors have detailed explanations: E0412, E0601.
For more information about an error, try `rustc --explain E0412`.
