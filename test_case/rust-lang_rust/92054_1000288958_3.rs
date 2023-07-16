
error: internal compiler error: /rustc/e100ec5bc7cd768ec17d75448b29c9ab4a39272b\compiler\rustc_codegen_ssa\src\mir\operand.rs:132:38: Deref of by-Ref operand OperandRef(Ref((%"alloc::boxed::Box<[u8], &alloc::alloc::Global>"*:  %4 = alloca %"alloc::boxed::Box<[u8], &alloc::alloc::Global>", align 8), None, Align 
{ pow2: 3 }) @ TyAndLayout { ty: std::boxed::Box<[u8], &std::alloc::Global>, layout: Layout { fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 16 }], memory_index: [0, 1] }, variants: Single { index: 0 }, abi: Aggregate { sized: true }, largest_niche: Some(Niche { offset: Size { raw: 0 }, scalar: Scalar { value: Pointer, valid_range: 1..=18446744073709551615 } }), align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 24 } } })

thread 'rustc' panicked at 'Box<dyn Any>', compiler\rustc_errors\src\lib.rs:1169:9
stack backtrace:
   0:     0x7fff776a9c4f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd920fc7772705f2f
   1:     0x7fff776d47fa - core::fmt::write::h6cae84e4480d77f7
   2:     0x7fff7769b8b8 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::h7935709480648778
   3:     0x7fff776ad3a6 - std::panicking::take_hook::h439fe9c3e72ee47e
   4:     0x7fff776ace89 - std::panicking::take_hook::h439fe9c3e72ee47e
   5:     0x7fff332896d6 - <rustc_lint[c17777233df79390]::BuiltinCombinedLateLintPass as rustc_lint[c17777233df79390]::passes::LateLintPass>::check_fn_post                                                                                                                                                            
   6:     0x7fff776adc09 - std::panicking::rust_panic_with_hook::hf0decd59f5351093
   7:     0x7fff37a828a0 - <rustc_errors[e666500dafc617f3]::diagnostic_builder::DiagnosticBuilder>::code
   8:     0x7fff37a82389 - <rustc_errors[e666500dafc617f3]::diagnostic_builder::DiagnosticBuilder>::code
   9:     0x7fff37d6daa1 - rustc_query_system[da29e76f6eb78471]::query::job::report_cycle
  10:     0x7fff37ab8040 - <rustc_errors[e666500dafc617f3]::registry::InvalidErrorCode as core[695968f75b490638]::fmt::Debug>::fmt
  11:     0x7fff37abf837 - <rustc_errors[e666500dafc617f3]::HandlerInner>::emit_diagnostic
  12:     0x7fff37abcf72 - <rustc_errors[e666500dafc617f3]::Handler>::bug
  13:     0x7fff37781421 - <rustc_middle[2b829493013aafeb]::ty::consts::valtree::ValTree>::zst
  14:     0x7fff37782eb9 - <rustc_middle[2b829493013aafeb]::ty::closure::UpvarBorrow as core[695968f75b490638]::fmt::Debug>::fmt
  15:     0x7fff37d63990 - rustc_middle[2b829493013aafeb]::util::bug::bug_fmt
  16:     0x7fff335e718b - <rustc_target[23893fd9125de42]::abi::TyAndLayout<&rustc_middle[2b829493013aafeb]::ty::TyS> as rustc_codegen_llvm[8c2bb20c9847d66e]::type_of::LayoutLlvmExt>::pointee_info_at
  17:     0x7fff3360a974 - <rustc_codegen_llvm[8c2bb20c9847d66e]::back::archive::LlvmArchiveBuilder as rustc_codegen_ssa[a10dd870132f8d9c]::back::archive::ArchiveBuilder>::inject_dll_import_lib
  18:     0x7fff335fa85d - <rustc_codegen_llvm[8c2bb20c9847d66e]::back::archive::LlvmArchiveBuilder as rustc_codegen_ssa[a10dd870132f8d9c]::back::archive::ArchiveBuilder>::inject_dll_import_lib
  19:     0x7fff335f4474 - <rustc_codegen_llvm[8c2bb20c9847d66e]::back::archive::LlvmArchiveBuilder as rustc_codegen_ssa[a10dd870132f8d9c]::back::archive::ArchiveBuilder>::inject_dll_import_lib
  20:     0x7fff33593c82 - <rustc_codegen_llvm[8c2bb20c9847d66e]::context::CodegenCx as rustc_codegen_ssa[a10dd870132f8d9c]::traits::type_::LayoutTypeMethods>::reg_backend_type
  21:     0x7fff335b203c - <rustc_codegen_llvm[8c2bb20c9847d66e]::LlvmCodegenBackend as rustc_codegen_ssa[a10dd870132f8d9c]::traits::backend::CodegenBackend>::link
  22:     0x7fff335445bf - <rustc_codegen_llvm[8c2bb20c9847d66e]::base::ValueIter as core[695968f75b490638]::iter::traits::iterator::Iterator>::next
  23:     0x7fff3350012e - <rustc_codegen_llvm[8c2bb20c9847d66e]::llvm_::archive_ro::Child as core[695968f75b490638]::ops::drop::Drop>::drop
  24:     0x7fff335428c1 - <rustc_codegen_llvm[8c2bb20c9847d66e]::base::ValueIter as core[695968f75b490638]::iter::traits::iterator::Iterator>::next
  25:     0x7fff335ae070 - <rustc_codegen_llvm[8c2bb20c9847d66e]::LlvmCodegenBackend as rustc_codegen_ssa[a10dd870132f8d9c]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7fff3349ba32 - <rustc_interface[539fe7db8d2f0cb0]::passes::boxed_resolver::BoxedResolver>::to_resolver_outputs
  27:     0x7fff333fba7b - <rustc_interface[539fe7db8d2f0cb0]::queries::Queries>::ongoing_codegen
  28:     0x7fff332afe80 - <rustc_middle[2b829493013aafeb]::ty::SymbolName as core[695968f75b490638]::fmt::Debug>::fmt
  29:     0x7fff332a3f85 - rustc_driver[cf92f2c00c4e0f9e]::pretty::print_after_hir_lowering
  30:     0x7fff332b2ab1 - <rustc_middle[2b829493013aafeb]::ty::SymbolName as core[695968f75b490638]::fmt::Debug>::fmt
  31:     0x7fff332ace53 - rustc_driver[cf92f2c00c4e0f9e]::pretty::print_after_hir_lowering
  32:     0x7fff33327c78 - <rustc_driver[cf92f2c00c4e0f9e]::args::Error as core[695968f75b490638]::fmt::Debug>::fmt
  33:     0x7fff776bb38c - std::sys::windows::thread::Thread::new::h2b78cd4067459812
  34:     0x7fffa73654e0 - BaseThreadInitThunk
  35:     0x7fffa7e6485b - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (e100ec5bc 2021-12-21) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
