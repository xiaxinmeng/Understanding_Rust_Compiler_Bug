
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { c
ode: 5, message: "Access is denied." } }', src\libcore\result.rs:860:4
stack backtrace:
   0: _rdl_shrink_in_place
   1: std::panicking::Location::column
   2: std::panicking::Location::column
   3: std::panicking::rust_panic_with_hook
   4: std::panicking::begin_panic_fmt
   5: std::panicking::begin_panic_fmt
   6: rust_begin_unwind
   7: core::panicking::panic_fmt
   8: <rustc_metadata::encoder::ImplVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeV
isitor<'v>>::visit_impl_item
   9: rustc_metadata::locator::Context::report_errs
  10: rustc_metadata::creader::CrateLoader::new
  11: rustc_metadata::creader::CrateLoader::new
  12: <rustc_metadata::creader::CrateLoader<'a> as rustc::middle::cstore::CrateLoader>::proce
ss_item
  13: rustc_resolve::build_reduced_graph::<impl rustc_resolve::ToNameBinding<'a> for (rustc::
hir::def::Def, rustc::ty::Visibility, syntax_pos::Span, syntax_pos::hygiene::Mark)>::to_name_
binding
  14: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit:
:Visitor<'a>>::visit_item
  15: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit:
:Visitor<'a>>::visit_item
  16: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a
>>::visit_expansion
  17: syntax::ext::expand::MacroExpander::expand_crate
  18: syntax::ext::expand::MacroExpander::expand_crate
  19: syntax::ext::expand::MacroExpander::expand_crate
  20: rustc_driver::driver::count_nodes
  21: rustc_driver::driver::count_nodes
  22: rustc_driver::driver::compile_input
  23: rustc_driver::run_compiler
  24: <unknown>
  25: _rust_maybe_catch_panic
  26: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'
v>>::visit_trait_item
  27: std::sys::imp::thread::Thread::new
  28: BaseThreadInitThunk
