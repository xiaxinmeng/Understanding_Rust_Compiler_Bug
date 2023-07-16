
error[E0572]: return statement outside of function body
 --> src\main.rs:2:10
  |
2 |       [(); return match () {
  |  __________^
3 | |         'c' => 1
4 | |     }];
  | |_____^

error: internal compiler error: librustc\ty\layout.rs:1122: LayoutDetails::compute: unexpected type `[type error]`

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
  14: rustc::ty::layout::provide
  15: rustc::ty::context::tls::track_diagnostic
  16: <rustc::ty::layout::LayoutError<'tcx> as core::fmt::Display>::fmt
  17: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  18: rustc::ty::context::tls::track_diagnostic
  19: rustc::ty::context::tls::track_diagnostic
  20: rustc::dep_graph::graph::DepGraph::assert_ignored
  21: rustc::ty::context::tls::track_diagnostic
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  24: <rustc::ty::layout::LayoutCx<'tcx, rustc::ty::context::TyCtxt<'a, 'tcx, 'tcx>> as rustc_target::abi::LayoutOf>::layout_of
  25: <rustc_mir::monomorphize::collector::MonoItemCollectionMode as core::fmt::Debug>::fmt
  26: rustc_mir::hair::pattern::compare_const_vals
  27: <rustc_mir::hair::pattern::_match::Matrix<'a, 'tcx> as core::fmt::Debug>::fmt
  28: <rustc_mir::hair::pattern::_match::Matrix<'a, 'tcx> as core::fmt::Debug>::fmt
  29: <rustc_mir::hair::pattern::_match::Matrix<'a, 'tcx> as core::fmt::Debug>::fmt
  30: <rustc_mir::transform::uniform_array_move_out::LocalUse as core::fmt::Debug>::fmt
  31: <rustc_mir::monomorphize::collector::MonoItemCollectionMode as core::fmt::Debug>::fmt
  32: <rustc_mir::hair::pattern::_match::Matrix<'a, 'tcx> as core::fmt::Debug>::fmt
  33: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  34: <rustc_mir::hair::pattern::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  35: <rustc_mir::dataflow::impls::borrows::Borrows<'a, 'gcx, 'tcx> as rustc_mir::dataflow::BitDenotation>::start_block_effect
  36: rustc_mir::hair::pattern::check_match::check_crate
  37: rustc::ty::context::tls::track_diagnostic
  38: rustc::ty::context::tls::track_diagnostic
  39: rustc::dep_graph::graph::DepGraph::assert_ignored
  40: rustc::ty::context::tls::track_diagnostic
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  42: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  43: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::check_match
  44: rustc_mir::interpret::const_eval::const_eval_provider
  45: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  46: rustc::ty::context::tls::track_diagnostic
  47: rustc::ty::context::tls::track_diagnostic
  48: rustc::dep_graph::graph::DepGraph::assert_ignored
  49: rustc::ty::context::tls::track_diagnostic
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  52: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::const_eval
  53: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  54: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  55: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  56: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  57: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  58: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  59: <rustc_typeck::check::GatherLocalsVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_pat
  60: <rustc_typeck::collect::has_late_bound_regions::LateBoundRegionsDetector<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_lifetime
  61: <rustc_typeck::check_unused::CollectExternCrateVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  62: <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  63: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  64: rustc::ty::context::tls::track_diagnostic
  65: rustc::ty::context::tls::track_diagnostic
  66: rustc::dep_graph::graph::DepGraph::assert_ignored
  67: rustc::ty::context::tls::track_diagnostic
  68: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  69: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  70: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  71: <rustc_typeck::check::GatherLocalsVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_fn
  72: <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  73: rustc::ty::context::tls::track_diagnostic
  74: rustc::ty::context::tls::track_diagnostic
  75: rustc::dep_graph::graph::DepGraph::assert_ignored
  76: rustc::ty::context::tls::track_diagnostic
  77: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  78: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  79: rustc_typeck::check_crate
  80: <humantime::date::Error as std::error::Error>::cause
  81: <rustc_driver::pretty::IdentifiedAnnotation<'hir> as rustc_driver::pretty::PrinterSupport>::sess
  82: <unknown>
  83: rustc_driver::driver::compile_input
  84: rustc_driver::run_compiler
  85: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  86: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  87: _rust_maybe_catch_panic
  88: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  89: rustc_driver::main
  90: <unknown>
  91: std::panicking::update_panic_count
  92: _rust_maybe_catch_panic
  93: std::rt::lang_start_internal
  94: <unknown>
  95: <unknown>
  96: BaseThreadInitThunk
  97: RtlUserThreadStart
query stack during panic:
#0 [layout_raw] computing layout of `[type error]`
#1 [check_match] processing `main::{{constant}}`
#2 [const_eval] const-evaluating `main::{{constant}}`
#3 [typeck_tables_of] processing `main`
#4 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0572`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (e3bf634e0 2018-06-28) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `unexpected_type`.

To learn more, run the command again with --verbose.
