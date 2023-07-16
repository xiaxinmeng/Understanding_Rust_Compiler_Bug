plain
thread 'rustc' panicked at 'Forcing query with already existing DepNode.
- query-key: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: (Const { ty: [u32; 4], val: ByRef(AllocId(4849).0x0, Allocation { bytes: [0, 0, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 2 }, mutability: Immutable, extra: () }) }, field[0]) }
- dep-node: const_field(fa069fcc6f4deb7b-95c6e5e89865a5b4)', src\librustc\ty\query\plumbing.rs:538:9
stack backtrace:
   0: std::sys_common::alloc::realloc_fallback
   1: std::panicking::take_hook
   2: std::panicking::take_hook
   3: <(rustc::hir::def_id::DefId, rustc::hir::def_id::DefId) as rustc::ty::query::keys::Key>::default_span
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic_fmt
   6: std::panicking::begin_panic_fmt
   7: <rustc_codegen_llvm::llv$u6d$_..ffi..debuginfo..DISPFlags$u20$as$u20$core..fmt..Debug$GT$::fmt
   8: <rustc_codegen_llvm::debuginfo::metadata::MemberDescription as core::fmt::Debug>::fmt
   9: <rustc_codegen_llvm::llv$u6d$_..ffi..debuginfo..DISPFlags$u20$as$u20$core..fmt..Debug$GT$::fmt
  10: <rustc_codegen_llvm::debuginfo::CrateDebugContext as core::ops::drop::Drop>::drop
  11: <rustc_codegen_llvm::debuginfo::metadata::MemberDescription as core::fmt::Debug>::fmt
  12: <rustc_codegen_llvm::llv$u6d$_..ffi..debuginfo..DISPFlags$u20$as$u20$core..fmt..Debug$GT$::fmt
  13: <rustc_codegen_llvm::debuginfo::CrateDebugContext as core::ops::drop::Drop>::drop
  14: <rustc_codegen_llvm::debuginfo::CrateDebugContext as core::ops::drop::Drop>::drop
  15: <rustc_codegen_llvm::llv$u6d$_..ffi..PassKind$u20$as$u20$core..fmt..Debug$GT$::fmt
  16: <rustc_codegen_llvm::back::lto::Linker as core::ops::drop::Drop>::drop
  17: <rustc_codegen_llvm::base::ValueIter as core::iter::traits::iterator::Iterator>::next
  18: <rustc_codegen_llvm::llv$u6d$_..OperandBundleDef$u20$as$u20$core..ops..drop..Drop$GT$::drop
  19: <rustc_codegen_llvm::base::ValueIter as core::iter::traits::iterator::Iterator>::next
  20: <rustc_codegen_llvm::llv$u6d$_..ffi..PassKind$u20$as$u20$core..fmt..Debug$GT$::fmt
  21: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  22: <rustc_interface::profile::trace::Query as core::fmt::Debug>::fmt
  23: rustc_interface::passes::BoxedResolver::to_expansion_result
  24: <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_trait_item
  25: rustc_interface::passes::BoxedResolver::complete
  26: <rustc_interface::profile::trace::Query as core::fmt::Debug>::fmt
  27: rustc_interface::passes::BoxedResolver::to_expansion_result
  28: <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_trait_item
  29: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  30: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  31: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  32: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  33: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  34: _rust_maybe_catch_panic
  35: <env_logger::fmt::WriteStyle as core::default::Default>::default
  36: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  37: std::sys::windows::thread::Thread::new
  38: BaseThreadInitThunk
  39: RtlUserThreadStart
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (b25ee6449 2019-06-17) running on x86_64-pc-windows-msvc
note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib
note: some of the compiler flags provided by cargo are hidden
