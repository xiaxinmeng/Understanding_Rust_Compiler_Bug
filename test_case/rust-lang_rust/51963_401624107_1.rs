
error[E0572]: return statement outside of function body
 --> src\main.rs:2:10
  |
2 |       [(); return match 1 {
  |  __________^
3 | |         1 => 1,
4 | |     }]
  | |_____^

error: internal compiler error: librustc_mir\hair\pattern\mod.rs:1172: impossible case reached

thread 'main' panicked at 'Box<Any>', librustc_errors\lib.rs:554:9
stack backtrace:
   0: <std::collections::hash::map::DefaultHasher as core::fmt::Debug>::fmt
   1: <std::sys::windows::dynamic_lib::DynamicLibrary as core::ops::drop::Drop>::drop
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: <rustc::ty::SymbolName as core::fmt::Debug>::fmt
   5: std::panicking::rust_panic_with_hook
   6: <rustc_errors::emitter::ColorConfig as core::fmt::Debug>::fmt
   7: rustc_errors::Handler::bug
   8: rustc::traits::util::Elaborator::filter_to_traits
   9: rustc::ty::context::tls::track_diagnostic
  10: rustc::ty::context::tls::track_diagnostic
  11: rustc::ty::context::tls::track_diagnostic
  12: rustc::session::bug_fmt
  13: rustc::session::bug_fmt
  14: rustc_mir::hair::pattern::compare_const_vals
  15: rustc_mir::hair::pattern::PatternContext::lower_pattern
  16: rustc_mir::hair::pattern::PatternContext::lower_pattern
  17: <rustc_mir::transform::inline::CallSite<'tcx> as core::fmt::Debug>::fmt
  18: <unknown>
  19: <unknown>
  20: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  21: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  22: <rustc_mir::dataflow::impls::borrows::Borrows<'a, 'gcx, 'tcx> as rustc_mir::dataflow::BitDenotation>::start_block_effect
  23: rustc_mir::hair::pattern::check_match::check_crate
  24: rustc::ty::context::tls::track_diagnostic
  25: rustc::ty::context::tls::track_diagnostic
  26: rustc::dep_graph::graph::DepGraph::assert_ignored
  27: rustc::ty::context::tls::track_diagnostic
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  30: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::check_match
  31: rustc_mir::interpret::const_eval::const_eval_provider
  32: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  33: rustc::ty::context::tls::track_diagnostic
  34: rustc::ty::context::tls::track_diagnostic
  35: rustc::dep_graph::graph::DepGraph::assert_ignored
  36: rustc::ty::context::tls::track_diagnostic
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  39: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::const_eval
  40: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  41: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  42: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  43: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  44: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  45: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  46: <rustc_typeck::check::GatherLocalsVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_pat
  47: <rustc_typeck::collect::has_late_bound_regions::LateBoundRegionsDetector<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_lifetime
  48: <rustc_typeck::check_unused::CollectExternCrateVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  49: <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  50: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  51: rustc::ty::context::tls::track_diagnostic
  52: rustc::ty::context::tls::track_diagnostic
  53: rustc::dep_graph::graph::DepGraph::assert_ignored
  54: rustc::ty::context::tls::track_diagnostic
  55: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  56: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  57: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  58: <rustc_typeck::check::GatherLocalsVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_fn
  59: <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  60: rustc::ty::context::tls::track_diagnostic
  61: rustc::ty::context::tls::track_diagnostic
  62: rustc::dep_graph::graph::DepGraph::assert_ignored
  63: rustc::ty::context::tls::track_diagnostic
  64: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  65: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  66: rustc_typeck::check_crate
  67: <humantime::date::Error as std::error::Error>::cause
  68: <rustc_driver::pretty::IdentifiedAnnotation<'hir> as rustc_driver::pretty::PrinterSupport>::sess
  69: <unknown>
  70: rustc_driver::driver::compile_input
  71: rustc_driver::run_compiler
  72: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  73: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  74: _rust_maybe_catch_panic
  75: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  76: rustc_driver::main
  77: <unknown>
  78: std::panicking::update_panic_count
  79: _rust_maybe_catch_panic
  80: std::rt::lang_start_internal
  81: <unknown>
  82: <unknown>
  83: BaseThreadInitThunk
  84: RtlUserThreadStart
query stack during panic:
#0 [check_match] processing `main::{{constant}}`
#1 [const_eval] const-evaluating `main::{{constant}}`
#2 [typeck_tables_of] processing `main`
#3 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0572`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (e3bf634e0 2018-06-28) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `match_impossible_case`.

