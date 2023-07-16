
error[E0697]: closures cannot be static
   --> src\lib.rs:154:16
    |
154 |         [(); &(static || {}) as *const _ as usize]
    |                ^^^^^^^^^

error[E0018]: raw pointers cannot be cast to integers in constants
   --> src\lib.rs:154:14
    |
154 |         [(); &(static || {}) as *const _ as usize]
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: librustc\ty\sty.rs:2008: expected constant usize, got Const {
    ty: usize,
    val: Scalar(
        Ptr(
            Pointer {
                alloc_id: AllocId(
                    1
                ),
                offset: Size {
                    raw: 0
                }
            }
        )
    )
}

thread 'main' panicked at 'Box<Any>', librustc_errors\lib.rs:554:9
stack backtrace:
   0: <std::sys::windows::args::Args as core::ops::drop::Drop>::drop
   1: std::error::<impl core::convert::From<alloc::borrow::Cow<'b, str>> for alloc::boxed::Box<(dyn std::error::Error + core::marker::Sync + core::marker::Send + 'a)>>::from
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: <rustc::ty::SymbolName as core::fmt::Display>::fmt
   5: std::panicking::rust_panic_with_hook
   6: <rustc_errors::emitter::ColorConfig as core::fmt::Debug>::fmt
   7: rustc_errors::Handler::bug
   8: rustc::session::config::Externs::new
   9: rustc::ty::context::tls::track_diagnostic
  10: rustc::ty::context::tls::track_diagnostic
  11: rustc::ty::context::tls::track_diagnostic
  12: rustc::session::bug_fmt
  13: rustc::session::bug_fmt
  14: rustc::ty::context::tls::track_diagnostic
  15: rustc::util::ppaux::<impl core::fmt::Debug for rustc::ty::sty::TraitRef<'tcx>>::fmt
  16: <rustc::util::common::ErrorReported as core::fmt::Debug>::fmt
  17: core::fmt::write
  18: alloc::fmt::format
  19: rustc::infer::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_region_errors
  20: rustc::infer::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_and_explain_type_error
  21: rustc::infer::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::note_type_err
  22: rustc::infer::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_and_explain_type_error
  23: rustc::infer::InferCtxt::report_mismatched_types
  24: <rustc_typeck::coherence::orphan::OrphanChecker<'cx, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  25: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  26: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  27: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  28: <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty
  29: <rustc_typeck::check::GatherLocalsVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_pat
  30: <rustc_typeck::collect::has_late_bound_regions::LateBoundRegionsDetector<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_lifetime
  31: <rustc_typeck::check::method::probe::ProbeScope as core::fmt::Debug>::fmt
  32: <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  33: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  34: rustc::ty::context::tls::track_diagnostic
  35: rustc::ty::context::tls::track_diagnostic
  36: rustc::dep_graph::graph::DepGraph::assert_ignored
  37: rustc::ty::context::tls::track_diagnostic
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  41: <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  42: rustc::ty::context::tls::track_diagnostic
  43: rustc::ty::context::tls::track_diagnostic
  44: rustc::dep_graph::graph::DepGraph::assert_ignored
  45: rustc::ty::context::tls::track_diagnostic
  46: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  48: rustc_typeck::check_crate
  49: <unknown>
  50: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  51: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  52: rustc_driver::driver::compile_input
  53: rustc_driver::run_compiler
  54: rustc_driver::target_features::add_configuration
  55: <unknown>
  56: _rust_maybe_catch_panic
  57: rustc_driver::profile::dump
  58: rustc_driver::main
  59: <unknown>
  60: std::panicking::update_panic_count
  61: _rust_maybe_catch_panic
  62: std::rt::lang_start_internal
  63: <unknown>
  64: <unknown>
  65: BaseThreadInitThunk
  66: RtlUserThreadStart
query stack during panic:
#0 [typeck_tables_of] processing `internal_compiler_errors::expected_const_got`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 3 previous errors

Some errors occurred: E0018, E0697.
For more information about an error, try `rustc --explain E0018`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.29.0-nightly (31f1bc7b4 2018-07-15) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `test_bugs`.
