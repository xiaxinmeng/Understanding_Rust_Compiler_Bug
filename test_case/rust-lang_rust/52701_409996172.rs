
warning: function cannot return without recurring
   --> src\lib.rs:173:5
    |
173 |     pub fn infinite_recursion() -> impl Fn() { infinite_recursion() }
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^   -------------------- recursive call site
    |     |
    |     cannot return without recurring
    |
    = note: #[warn(unconditional_recursion)] on by default
    = help: a `loop` may express intention better if this is on purpose

thread 'main' panicked at 'assertion failed: `(left != right)`
  left: `impl std::ops::Fn<()>`,
 right: `impl std::ops::Fn<()>`: infinite recursion', librustc\traits\query\normalize.rs:130:25
stack backtrace:
   0: <std::sync::mpsc::select::Select as core::fmt::Debug>::fmt
   1: std::stdsimd::arch::detect::os::check_for
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: <rustc::ty::query::on_disk_cache::CacheEncoder<'enc, 'a, 'tcx, serialize::opaque::Encoder<'enc>> as serialize::serialize::SpecializedEncoder<rustc::ich::fingerprint::Fingerprint>>::specialized_encode
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic_fmt
   7: std::panicking::begin_panic_fmt
   8: <rustc::traits::query::normalize::QueryNormalizer<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
   9: rustc_traits::provide
  10: <rustc_traits::chalk_context::ConstrainedSubst<'a> as rustc::ty::context::Lift<'tcx>>::lift_to_tcx
  11: <rustc::ty::Predicate<'tcx> as rustc_traits::lowering::Lower<rustc::ty::sty::Binder<rustc::traits::DomainGoal<'tcx>>>>::lower
  12: <unknown>
  13: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  14: rustc::ty::context::tls::track_diagnostic
  15: rustc::ty::context::tls::track_diagnostic
  16: rustc::dep_graph::graph::DepGraph::assert_ignored
  17: rustc::ty::context::tls::track_diagnostic
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  20: <rustc::traits::query::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder<'cx, 'tcx> as rustc::ty::fold::TypeFolder<'tcx, 'tcx>>::fold_ty
  21: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  22: rustc_mir::monomorphize::collector::collect_crate_mono_items
  23: rustc_mir::monomorphize::collector::collect_crate_mono_items
  24: rustc_mir::monomorphize::collector::collect_crate_mono_items
  25: <rustc_codegen_llvm::time_graph::TimelineId as core::fmt::Debug>::fmt
  26: <rustc_codegen_llvm::base::ValueIter as core::iter::iterator::Iterator>::next
  27: rustc::ty::context::tls::track_diagnostic
  28: rustc::ty::context::tls::track_diagnostic
  29: rustc::dep_graph::graph::DepGraph::assert_ignored
  30: rustc::ty::context::tls::track_diagnostic
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  33: <rustc_codegen_llvm::back::linker::MsvcLinker<'a> as rustc_codegen_llvm::back::linker::Linker>::finalize
  34: rustc::ty::context::tls::track_diagnostic
  35: rustc::ty::context::tls::track_diagnostic
  36: rustc::dep_graph::graph::DepGraph::assert_ignored
  37: rustc::ty::context::tls::track_diagnostic
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  40: <rustc_metadata::encoder::ImplVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  41: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  42: rustc::ty::context::TyCtxt::encode_metadata
  43: rustc_codegen_llvm::base::codegen_instance
  44: <rustc_codegen_llvm::time_graph::TimelineId as core::fmt::Debug>::fmt
  45: <rustc_codegen_llvm::base::ValueIter as core::iter::iterator::Iterator>::next
  46: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  47: rustc_driver::driver::build_output_filenames
  48: rustc_driver::driver::phase_4_codegen
  49: rustc_driver::profile::dump
  50: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  51: <rustc_driver::pretty::IdentifiedAnnotation<'hir> as rustc_driver::pretty::PrinterSupport>::sess
  52: <unknown>
  53: rustc_driver::driver::compile_input
  54: rustc_driver::run_compiler
  55: rustc_driver::target_features::add_configuration
  56: <rustc_driver::pretty::IdentifiedAnnotation<'hir> as rustc_driver::pretty::PrinterSupport>::sess
  57: _rust_maybe_catch_panic
  58: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  59: rustc_driver::main
  60: <unknown>
  61: std::panicking::update_panic_count
  62: _rust_maybe_catch_panic
  63: std::rt::lang_start_internal
  64: <unknown>
  65: <unknown>
  66: BaseThreadInitThunk
  67: RtlUserThreadStart
query stack during panic:
#0 [normalize_ty_after_erasing_regions] normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: impl std::ops::Fn<()> }`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] exported_symbols
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0 (9634041f0 2018-07-30) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `test_bugs`.

To learn more, run the command again with --verbose.
