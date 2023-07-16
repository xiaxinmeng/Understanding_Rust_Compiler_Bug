
time: 2018-09-18 14:07:02.708000
invoking: cargo +stable-x86_64 build --release
in: c:\jenkins\workspace\SDK5\build-local-system\giles
   Compiling slab v0.3.0
thread 'main' panicked at 'specified instant was later than self', libcore\option.rs:960:5
   Compiling ucd-util v0.1.1
   Compiling lazy_static v0.2.11
   
stack backtrace:
   0: <std::sync::mpsc::select::Select as core::fmt::Debug>::fmt
   1: std::stdsimd::arch::detect::os::check_for
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: <rustc::ty::query::on_disk_cache::CacheEncoder<'enc, 'a, 'tcx, serialize::opaque::Encoder<'enc>> as serialize::serialize::SpecializedEncoder<rustc::ich::fingerprint::Fingerprint>>::specialized_encode
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::option::expect_failed
  10: <std::sys_common::process::DefaultEnvKey as core::fmt::Debug>::fmt
  11: std::time::Instant::elapsed
  12: <rustc_codegen_utils::codegen_backend::MetadataOnlyCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  13: rustc_codegen_utils::symbol_names::provide
  14: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x>>::read_str
  15: rustc::ty::context::tls::track_diagnostic
  16: rustc::dep_graph::graph::DepGraph::assert_ignored
  17: rustc::ty::context::tls::track_diagnostic
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  20: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::symbol_name
  21: <rustc_codegen_llvm::back::write::MainThreadWorkerState as core::fmt::Debug>::fmt
  22: <rustc_codegen_llvm::debuginfo::create_scope_map::MirDebugScope as core::fmt::Debug>::fmt
  23: <rustc_codegen_llvm::back::link::exec_linker::Escape<'a> as core::fmt::Display>::fmt
  24: <rustc_codegen_llvm::base::ValueIter as core::iter::iterator::Iterator>::next
  25: rustc::ty::context::tls::track_diagnostic
  26: rustc::dep_graph::graph::DepGraph::assert_ignored
  27: rustc::ty::context::tls::track_diagnostic
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_print_query_stack
  30: <rustc_codegen_llvm::base::ValueIter as core::iter::iterator::Iterator>::next
  31: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  32: rustc_driver::driver::build_output_filenames
  33: rustc_driver::driver::phase_4_codegen
  34: rustc_driver::profile::dump
  35: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  36: <rustc_driver::pretty::IdentifiedAnnotation<'hir> as rustc_driver::pretty::PrinterSupport>::sess
  37: <unknown>
  38: rustc_driver::driver::compile_input
  39: rustc_driver::run_compiler
  40: rustc_driver::target_features::add_configuration
  41: <rustc_driver::pretty::IdentifiedAnnotation<'hir> as rustc_driver::pretty::PrinterSupport>::sess
  42: _rust_maybe_catch_panic
  43: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::  44: rustc_driver::main
  45: <unknown>
  46: std::panicking::update_panic_count
  47: _rust_maybe_catch_panic
  48: std::rt::lang_start_internal
  49: <unknown>
  50: <unknown>
  51: BaseThreadInitThunk
  52: RtlUserThreadStart
  
query stack during panic:
#0 [symbol_name] computing the symbol for `<alloc::btree::node::Handle<alloc::btree::node::NodeRef<BorrowType, K, V, NodeType>, HandleType>><alloc::btree::node::marker::Owned, std::sys::windows::process::WindowsEnvKey, std::option::Option<std::ffi::OsString>, alloc::btree::node::marker::Leaf, alloc::btree::node::marker::KV>::reborrow`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0 (9634041f0 2018-07-30) running on x86_64-pc-windows-msvc

note: compiler flags: -C opt-level=3 --crate-type bin

note: some of the compiler flags provided by cargo are hidden
