
> rustc .\src\main.rs
warning: field `execute` is never read
  --> .\src\main.rs:10:5
   |
9  | struct Instruction {
   |        ----------- field in this struct
10 |     execute: fn(),
   |     ^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

error: internal compiler error: compiler\rustc_codegen_llvm\src\context.rs:974:13: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
  --> .\src\main.rs:13:1
   |
13 | fn main() {
   | ^^^^^^^^^

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52\compiler\rustc_errors\src\lib.rs:1332:9
stack backtrace:
   0:     0x7ffaea079fbf - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf958371dc84a30d1
   1:     0x7ffaea0b4b5a - core::fmt::write::h8bc5f8dfcde4777f
   2:     0x7ffaea06c749 - <std::io::IoSlice as core::fmt::Debug>::fmt::hab1bb118c5bfde47
   3:     0x7ffaea07d8bb - std::panicking::default_hook::hcfea80c086f9466a
   4:     0x7ffaea07d535 - std::panicking::default_hook::hcfea80c086f9466a
   5:     0x7ffae547c404 - rustc_driver[a0a607043376aa59]::pretty::print_after_hir_lowering
   6:     0x7ffaea07e062 - std::panicking::rust_panic_with_hook::h43b18a7cf7089063
   7:     0x7ffae5571203 - rustc_codegen_llvm[293115cdcc5d4626]::llvm_::last_error
   8:     0x7ffae5570e99 - rustc_codegen_llvm[293115cdcc5d4626]::llvm_::last_error
   9:     0x7ffae5557f59 - <rustc_data_structures[34e88743362a5251]::profiling::EventFilter as core[f4e460384c48c425]::fmt::UpperHex>::fmt
  10:     0x7ffae55985e9 - <rustc_codegen_llvm[293115cdcc5d4626]::debuginfo::utils::FatPtrKind as core[f4e460384c48c425]::fmt::Debug>::fmt
::fmt::Debug>::fmt
  12:     0x7ffae55968d2 - <rustc_codegen_llvm[293115cdcc5d4626]::debuginfo::utils::FatPtrKind as core[f4e460384c48c425]::fmt::Debug>::fmt
  13:     0x7ffae55583f2 - <rustc_data_structures[34e88743362a5251]::profiling::EventFilter as core[f4e460384c48c425]::fmt::UpperHex>::fmt
  14:     0x7ffae5558298 - <rustc_data_structures[34e88743362a5251]::profiling::EventFilter as core[f4e460384c48c425]::fmt::UpperHex>::fmt
  15:     0x7ffae5558256 - <rustc_data_structures[34e88743362a5251]::profiling::EventFilter as core[f4e460384c48c425]::fmt::UpperHex>::fmt
  16:     0x7ffae5589d7f - <rustc_codegen_llvm[293115cdcc5d4626]::builder::Builder as rustc_codegen_ssa[4b4bd3c4a79f8ce6]::traits::builder::BuilderMethods>::va_arg
  17:     0x7ffae5583ff8 - <rustc_target[2fd88d7eff8074c7]::abi::TyAndLayout<rustc_middle[345e23a01d15c30c]::ty::Ty> as rustc_codegen_llvm[293115cdcc5d4626]::type_of::LayoutLlvmExt>::pointee_info_at
  18:     0x7ffae3db2851 - <rustc_codegen_llvm[293115cdcc5d4626]::llvm_::OperandBundleDef as core[f4e460384c48c425]::ops::drop::Drop>::drop
  19:     0x7ffae3de88cb - <rustc_codegen_llvm[293115cdcc5d4626]::context::CodegenCx as rustc_codegen_ssa[4b4bd3c4a79f8ce6]::traits::type_::LayoutTypeMethods>::fn_decl_backend_type
  20:     0x7ffae3d8efad - <rustc_codegen_llvm[293115cdcc5d4626]::llvm_::ffi::Metadata as core[f4e460384c48c425]::cmp::PartialEq>::eq
  21:     0x7ffae2e976e0 - rustc_interface[e31b1d572a2c621e]::passes::analysis
  22:     0x7ffae2eb3eb9 - <rustc_codegen_llvm[293115cdcc5d4626]::base::ValueIter as core[f4e460384c48c425]::iter::traits::iterator::Iterator>::next
  23:     0x7ffae2ead307 - <rustc_codegen_llvm[293115cdcc5d4626]::LlvmCodegenBackend as rustc_codegen_ssa[4b4bd3c4a79f8ce6]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7ffae2e751c0 - rustc_interface[e31b1d572a2c621e]::proc_macro_decls::provide
  25:     0x7ffae2e84c1b - <rustc_interface[e31b1d572a2c621e]::passes::LintStoreExpandImpl as rustc_expand[7e539c7dccfd7e8f]::base::LintStoreExpand>::pre_expansion_lint
  26:     0x7ffae2e6e649 - <rustc_interface[e31b1d572a2c621e]::queries::Queries>::ongoing_codegen
  27:     0x7ffae2e2bdbc - <unknown>
  28:     0x7ffae2e23110 - <unknown>
  29:     0x7ffae2e2cdf0 - <unknown>
  30:     0x7ffae2e58cb9 - rustc_driver[a0a607043376aa59]::args::arg_expand_all
  31:     0x7ffae2e483bd - <unknown>
  32:     0x7ffaea08ebdc - std::sys::windows::thread::Thread::new::hdf946fbfa5577747
  33:     0x7ffb46107034 - BaseThreadInitThunk
  34:     0x7ffb479826a1 - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0 (a55dd71d5 2022-09-19) running on x86_64-pc-windows-msvc

query stack during panic:
end of query stack
error: aborting due to previous error; 1 warning emitted
