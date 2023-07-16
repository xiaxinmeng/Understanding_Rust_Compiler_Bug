
---- [ui] src/test/ui/box/issue-95036.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/box/issue-95036.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/issue-95036" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/issue-95036/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:125:32: deref of non-pointer OperandRef(Immediate((i8*:  %23 = load i8*, i8** %22, align 8, !nonnull !3, !noundef !3)) @ TyAndLayout { ty: std::ptr::NonNull<std::mem::MaybeUninit<[u8; 1]>>, layout: Layout { fields: Arbitrary { offsets: [Size { raw: 0 }], memory_index: [0] }, variants: Single { index: 0 }, abi: Scalar(Initialized { value: Pointer, valid_range: 1..=18446744073709551615 }), largest_niche: Some(Niche { offset: Size { raw: 0 }, value: Pointer, valid_range: 1..=18446744073709551615 }), align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 8 } } })

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1349:9
stack backtrace:
   0:     0x7f63758681bc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd8aca151e67c8931
   1:     0x7f63758cc318 - core::fmt::write::hda5d9e6a9927afdc
   2:     0x7f6375858501 - std::io::Write::write_fmt::h6e9facd7b3f1e45b
   3:     0x7f637586b1de - std::panicking::default_hook::{{closure}}::h6f10d36904bdbc6e
   4:     0x7f637586ae14 - std::panicking::default_hook::hede5b607fbf63d68
   5:     0x7f63763b7d41 - rustc_driver[ca2c46528bdd3c4e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f637586ba3e - std::panicking::rust_panic_with_hook::h5e10068a4c2eb4c4
   7:     0x7f6378cd16e3 - std[fe45e4324dca75f5]::panicking::begin_panic::<rustc_errors[2766f829769489bc]::ExplicitBug>::{closure#0}
   8:     0x7f6378cd12a6 - std[fe45e4324dca75f5]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe45e4324dca75f5]::panicking::begin_panic<rustc_errors[2766f829769489bc]::ExplicitBug>::{closure#0}, !>
   9:     0x7f63762d8eaf - std[fe45e4324dca75f5]::panicking::begin_panic::<rustc_errors[2766f829769489bc]::ExplicitBug>
  10:     0x7f6378aca546 - std[fe45e4324dca75f5]::panic::panic_any::<rustc_errors[2766f829769489bc]::ExplicitBug>
  11:     0x7f6378abd196 - <rustc_errors[2766f829769489bc]::HandlerInner>::bug::<&alloc[978f5700631692ba]::string::String>
  12:     0x7f6378abce50 - <rustc_errors[2766f829769489bc]::Handler>::bug::<&alloc[978f5700631692ba]::string::String>
  13:     0x7f6378ccbed5 - rustc_middle[8d48c6df689d05d1]::util::bug::opt_span_bug_fmt::<rustc_span[5a3e90cbbc7c42a5]::span_encoding::Span>::{closure#0}
  14:     0x7f6378cc8edb - rustc_middle[8d48c6df689d05d1]::ty::context::tls::with_opt::<rustc_middle[8d48c6df689d05d1]::util::bug::opt_span_bug_fmt<rustc_span[5a3e90cbbc7c42a5]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f6378cc8e8c - rustc_middle[8d48c6df689d05d1]::ty::context::tls::with_opt::<rustc_middle[8d48c6df689d05d1]::util::bug::opt_span_bug_fmt<rustc_span[5a3e90cbbc7c42a5]::span_encoding::Span>::{closure#0}, ()>
  16:     0x7f6378ccbe19 - rustc_middle[8d48c6df689d05d1]::util::bug::opt_span_bug_fmt::<rustc_span[5a3e90cbbc7c42a5]::span_encoding::Span>
  17:     0x7f63762df91e - rustc_middle[8d48c6df689d05d1]::util::bug::bug_fmt
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  18:     0x7f63765659f8 - <rustc_codegen_ssa[f10137ac22e0f29f]::mir::operand::OperandRef<&rustc_codegen_llvm[87c8104e5bc925bf]::llvm_::ffi::Value>>::deref::<rustc_codegen_llvm[87c8104e5bc925bf]::context::CodegenCx>::{closure#0}
  19:     0x7f6376560bc6 - <rustc_codegen_ssa[f10137ac22e0f29f]::mir::operand::OperandRef<&rustc_codegen_llvm[87c8104e5bc925bf]::llvm_::ffi::Value>>::deref::<rustc_codegen_llvm[87c8104e5bc925bf]::context::CodegenCx>
  20:     0x7f63766d57ed - <rustc_codegen_ssa[f10137ac22e0f29f]::mir::FunctionCx<rustc_codegen_llvm[87c8104e5bc925bf]::builder::Builder>>::codegen_place
  21:     0x7f63766c8648 - <rustc_codegen_ssa[f10137ac22e0f29f]::mir::FunctionCx<rustc_codegen_llvm[87c8104e5bc925bf]::builder::Builder>>::codegen_rvalue_operand
  22:     0x7f63766d0ff7 - <rustc_codegen_ssa[f10137ac22e0f29f]::mir::FunctionCx<rustc_codegen_llvm[87c8104e5bc925bf]::builder::Builder>>::codegen_block
  23:     0x7f63766c4426 - rustc_codegen_ssa[f10137ac22e0f29f]::mir::codegen_mir::<rustc_codegen_llvm[87c8104e5bc925bf]::builder::Builder>
  24:     0x7f63765453b6 - rustc_codegen_ssa[f10137ac22e0f29f]::base::codegen_instance::<rustc_codegen_llvm[87c8104e5bc925bf]::builder::Builder>
  25:     0x7f637669dff7 - <rustc_middle[8d48c6df689d05d1]::mir::mono::MonoItem as rustc_codegen_ssa[f10137ac22e0f29f]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[87c8104e5bc925bf]::builder::Builder>
  26:     0x7f63765ef6c4 - rustc_codegen_llvm[87c8104e5bc925bf]::base::compile_codegen_unit::module_codegen
  27:     0x7f63765ed977 - rustc_codegen_llvm[87c8104e5bc925bf]::base::compile_codegen_unit
  28:     0x7f6376543dcb - rustc_codegen_ssa[f10137ac22e0f29f]::base::codegen_crate::<rustc_codegen_llvm[87c8104e5bc925bf]::LlvmCodegenBackend>
  29:     0x7f637662a599 - <rustc_codegen_llvm[87c8104e5bc925bf]::LlvmCodegenBackend as rustc_codegen_ssa[f10137ac22e0f29f]::traits::backend::CodegenBackend>::codegen_crate
  30:     0x7f63764b1901 - <rustc_session[f527fa7acd786cb5]::session::Session>::time::<alloc[978f5700631692ba]::boxed::Box<dyn core[9d0d73d25bc79eb6]::any::Any>, rustc_interface[6039aff59812a2a9]::passes::start_codegen::{closure#0}>
  31:     0x7f63764ae352 - <rustc_interface[6039aff59812a2a9]::queries::Queries>::ongoing_codegen
  32:     0x7f6376341a35 - <rustc_interface[6039aff59812a2a9]::interface::Compiler>::enter::<rustc_driver[ca2c46528bdd3c4e]::run_compiler::{closure#1}::{closure#2}, core[9d0d73d25bc79eb6]::result::Result<core[9d0d73d25bc79eb6]::option::Option<rustc_interface[6039aff59812a2a9]::queries::Linker>, rustc_errors[2766f829769489bc]::ErrorGuaranteed>>
  33:     0x7f63763a4d3b - rustc_span[5a3e90cbbc7c42a5]::with_source_map::<core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>, rustc_interface[6039aff59812a2a9]::interface::create_compiler_and_run<core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>, rustc_driver[ca2c46528bdd3c4e]::run_compiler::{closure#1}>::{closure#1}>
  34:     0x7f6376342c32 - <scoped_tls[f83357d34eb529bc]::ScopedKey<rustc_span[5a3e90cbbc7c42a5]::SessionGlobals>>::set::<rustc_interface[6039aff59812a2a9]::interface::run_compiler<core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>, rustc_driver[ca2c46528bdd3c4e]::run_compiler::{closure#1}>::{closure#0}, core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>>
  35:     0x7f63763a21d9 - std[fe45e4324dca75f5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6039aff59812a2a9]::util::run_in_thread_pool_with_globals<rustc_interface[6039aff59812a2a9]::interface::run_compiler<core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>, rustc_driver[ca2c46528bdd3c4e]::run_compiler::{closure#1}>::{closure#0}, core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>>::{closure#0}, core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>>
  36:     0x7f637635c271 - std[fe45e4324dca75f5]::panicking::try::<core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>, core[9d0d73d25bc79eb6]::panic::unwind_safe::AssertUnwindSafe<<std[fe45e4324dca75f5]::thread::Builder>::spawn_unchecked_<rustc_interface[6039aff59812a2a9]::util::run_in_thread_pool_with_globals<rustc_interface[6039aff59812a2a9]::interface::run_compiler<core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>, rustc_driver[ca2c46528bdd3c4e]::run_compiler::{closure#1}>::{closure#0}, core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>>::{closure#0}, core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x7f637639d1d2 - <<std[fe45e4324dca75f5]::thread::Builder>::spawn_unchecked_<rustc_interface[6039aff59812a2a9]::util::run_in_thread_pool_with_globals<rustc_interface[6039aff59812a2a9]::interface::run_compiler<core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>, rustc_driver[ca2c46528bdd3c4e]::run_compiler::{closure#1}>::{closure#0}, core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>>::{closure#0}, core[9d0d73d25bc79eb6]::result::Result<(), rustc_errors[2766f829769489bc]::ErrorGuaranteed>>::{closure#1} as core[9d0d73d25bc79eb6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f63758767f3 - std::sys::unix::thread::Thread::new::thread_start::h42e16de5fda4c2d6
  39:     0x7f636fdc9609 - start_thread
  40:     0x7f63756dc163 - clone
  41:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (a81536072 2022-04-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
