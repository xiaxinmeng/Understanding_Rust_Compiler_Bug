plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling thread_local v1.1.4
   Compiling matchers v0.1.0
   Compiling libloading v0.7.4
   Compiling xflags-macros v0.3.1
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs:180:17: Operand path should have handled transmute from `Ref` OperandRef(Ref(([16 x i8]*:  %1 = alloca [16 x i8], align 1), None, Align(1 bytes)) @ TyAndLayout { ty: [u8; 16], layout: Layout { size: Size(16 bytes), align: AbiAndPrefAlign { abi: Align(1 bytes), pref: Align(1 bytes) }, abi: Aggregate { sized: true }, fields: Array { stride: Size(1 bytes), count: 16 }, largest_niche: None, variants: Single { index: 0 } } }) to place PlaceRef { llval: ({ i64, i64 }*:  %2 = alloca { i64, i64 }, align 8), llextra: None, layout: TyAndLayout { ty: libc::timespec, layout: Layout { size: Size(16 bytes), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, abi: ScalarPair(Initialized { value: Int(I64, true), valid_range: 0..=18446744073709551615 }, Initialized { value: Int(I64, true), valid_range: 0..=18446744073709551615 }), fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], memory_index: [0, 1] }, largest_niche: None, variants: Single { index: 0 } } }, align: Align(8 bytes) }
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
stack backtrace:
   0:     0x7f28c4f67385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h37b9bda37ab512c9
   1:     0x7f28c4fd5a68 - core::fmt::write::hfd634242f591db07
   2:     0x7f28c4f5b5d1 - std::io::Write::write_fmt::h6c6aa91a40f27ef4
   3:     0x7f28c4f67191 - std::sys_common::backtrace::print::h0e8084f69c9d0c72
   4:     0x7f28c4f6a654 - std::panicking::default_hook::{{closure}}::h606432c76885abbb
   5:     0x7f28c4f6a320 - std::panicking::default_hook::h64b915259f934ab0
   6:     0x7f28c5a93c05 - rustc_driver_impl[264f97dffe570d0a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f28c4f6adb1 - std::panicking::rust_panic_with_hook::h04961300a5daed9c
   8:     0x7f28c88e2973 - std[12d3a7d2c76df072]::panicking::begin_panic::<rustc_errors[1ae224ebd3c23f26]::ExplicitBug>::{closure#0}
   9:     0x7f28c88de276 - std[12d3a7d2c76df072]::sys_common::backtrace::__rust_end_short_backtrace::<std[12d3a7d2c76df072]::panicking::begin_panic<rustc_errors[1ae224ebd3c23f26]::ExplicitBug>::{closure#0}, !>
  10:     0x7f28c5a395c6 - std[12d3a7d2c76df072]::panicking::begin_panic::<rustc_errors[1ae224ebd3c23f26]::ExplicitBug>
  11:     0x7f28c891ec10 - <rustc_errors[1ae224ebd3c23f26]::HandlerInner>::bug::<&alloc[5588931675677432]::string::String>
  12:     0x7f28c891e8a0 - <rustc_errors[1ae224ebd3c23f26]::Handler>::bug::<&alloc[5588931675677432]::string::String>
  13:     0x7f28c8991dec - rustc_middle[efb3a6c708c21be3]::util::bug::opt_span_bug_fmt::<rustc_span[5532f340690beab3]::span_encoding::Span>::{closure#0}
  14:     0x7f28c8991a4c - rustc_middle[efb3a6c708c21be3]::ty::context::tls::with_opt::<rustc_middle[efb3a6c708c21be3]::util::bug::opt_span_bug_fmt<rustc_span[5532f340690beab3]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f28c89919da - rustc_middle[efb3a6c708c21be3]::ty::context::tls::with_context_opt::<rustc_middle[efb3a6c708c21be3]::ty::context::tls::with_opt<rustc_middle[efb3a6c708c21be3]::util::bug::opt_span_bug_fmt<rustc_span[5532f340690beab3]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f28c8991d29 - rustc_middle[efb3a6c708c21be3]::util::bug::opt_span_bug_fmt::<rustc_span[5532f340690beab3]::span_encoding::Span>
  17:     0x7f28c5a39575 - rustc_middle[efb3a6c708c21be3]::util::bug::bug_fmt
  18:     0x7f28c5db14d9 - <rustc_codegen_ssa[53c6938fcfa63ab8]::mir::FunctionCx<rustc_codegen_llvm[4c75d8d6ef3bdec2]::builder::Builder>>::codegen_rvalue
  19:     0x7f28c5dab239 - rustc_codegen_ssa[53c6938fcfa63ab8]::mir::codegen_mir::<rustc_codegen_llvm[4c75d8d6ef3bdec2]::builder::Builder>
  20:     0x7f28c5d2ea17 - rustc_codegen_ssa[53c6938fcfa63ab8]::base::codegen_instance::<rustc_codegen_llvm[4c75d8d6ef3bdec2]::builder::Builder>
  21:     0x7f28c5d711d5 - <rustc_middle[efb3a6c708c21be3]::mir::mono::MonoItem as rustc_codegen_ssa[53c6938fcfa63ab8]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[4c75d8d6ef3bdec2]::builder::Builder>
  22:     0x7f28c5c5dc05 - rustc_codegen_llvm[4c75d8d6ef3bdec2]::base::compile_codegen_unit::module_codegen
  23:     0x7f28c5c61d61 - <rustc_codegen_llvm[4c75d8d6ef3bdec2]::LlvmCodegenBackend as rustc_codegen_ssa[53c6938fcfa63ab8]::traits::backend::ExtraBackendMethods>::compile_codegen_unit
  24:     0x7f28c5d2de3d - rustc_codegen_ssa[53c6938fcfa63ab8]::base::codegen_crate::<rustc_codegen_llvm[4c75d8d6ef3bdec2]::LlvmCodegenBackend>
  25:     0x7f28c5c66c14 - <rustc_codegen_llvm[4c75d8d6ef3bdec2]::LlvmCodegenBackend as rustc_codegen_ssa[53c6938fcfa63ab8]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7f28c5bd7d6f - <rustc_session[9e6aeb6041998cc]::session::Session>::time::<alloc[5588931675677432]::boxed::Box<dyn core[f31c9ce9be058123]::any::Any>, rustc_interface[799eb579dd4e3ad]::passes::start_codegen::{closure#0}>
  27:     0x7f28c5b531e8 - rustc_interface[799eb579dd4e3ad]::passes::start_codegen
  28:     0x7f28c5bf5a73 - <std[12d3a7d2c76df072]::thread::local::LocalKey<core[f31c9ce9be058123]::cell::Cell<*const ()>>>::with::<rustc_middle[efb3a6c708c21be3]::ty::context::tls::enter_context<<rustc_middle[efb3a6c708c21be3]::ty::context::GlobalCtxt>::enter<<rustc_interface[799eb579dd4e3ad]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f31c9ce9be058123]::result::Result<alloc[5588931675677432]::boxed::Box<dyn core[f31c9ce9be058123]::any::Any>, rustc_span[5532f340690beab3]::ErrorGuaranteed>>::{closure#0}, core[f31c9ce9be058123]::result::Result<alloc[5588931675677432]::boxed::Box<dyn core[f31c9ce9be058123]::any::Any>, rustc_span[5532f340690beab3]::ErrorGuaranteed>>::{closure#0}, core[f31c9ce9be058123]::result::Result<alloc[5588931675677432]::boxed::Box<dyn core[f31c9ce9be058123]::any::Any>, rustc_span[5532f340690beab3]::ErrorGuaranteed>>
  29:     0x7f28c5b55dcd - <rustc_middle[efb3a6c708c21be3]::ty::context::GlobalCtxt>::enter::<<rustc_interface[799eb579dd4e3ad]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f31c9ce9be058123]::result::Result<alloc[5588931675677432]::boxed::Box<dyn core[f31c9ce9be058123]::any::Any>, rustc_span[5532f340690beab3]::ErrorGuaranteed>>
  30:     0x7f28c5c28874 - <rustc_interface[799eb579dd4e3ad]::queries::Queries>::ongoing_codegen
  31:     0x7f28c5acacb2 - <rustc_interface[799eb579dd4e3ad]::interface::Compiler>::enter::<rustc_driver_impl[264f97dffe570d0a]::run_compiler::{closure#1}::{closure#2}, core[f31c9ce9be058123]::result::Result<core[f31c9ce9be058123]::option::Option<rustc_interface[799eb579dd4e3ad]::queries::Linker>, rustc_span[5532f340690beab3]::ErrorGuaranteed>>
  32:     0x7f28c5a94ec8 - rustc_span[5532f340690beab3]::set_source_map::<core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>, rustc_interface[799eb579dd4e3ad]::interface::run_compiler<core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>, rustc_driver_impl[264f97dffe570d0a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  33:     0x7f28c5aa3890 - std[12d3a7d2c76df072]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[799eb579dd4e3ad]::util::run_in_thread_pool_with_globals<rustc_interface[799eb579dd4e3ad]::interface::run_compiler<core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>, rustc_driver_impl[264f97dffe570d0a]::run_compiler::{closure#1}>::{closure#0}, core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>>
  34:     0x7f28c5acb4f6 - std[12d3a7d2c76df072]::panicking::try::<core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>, core[f31c9ce9be058123]::panic::unwind_safe::AssertUnwindSafe<<std[12d3a7d2c76df072]::thread::Builder>::spawn_unchecked_<rustc_interface[799eb579dd4e3ad]::util::run_in_thread_pool_with_globals<rustc_interface[799eb579dd4e3ad]::interface::run_compiler<core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>, rustc_driver_impl[264f97dffe570d0a]::run_compiler::{closure#1}>::{closure#0}, core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f28c5a994c5 - <<std[12d3a7d2c76df072]::thread::Builder>::spawn_unchecked_<rustc_interface[799eb579dd4e3ad]::util::run_in_thread_pool_with_globals<rustc_interface[799eb579dd4e3ad]::interface::run_compiler<core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>, rustc_driver_impl[264f97dffe570d0a]::run_compiler::{closure#1}>::{closure#0}, core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f31c9ce9be058123]::result::Result<(), rustc_span[5532f340690beab3]::ErrorGuaranteed>>::{closure#1} as core[f31c9ce9be058123]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f28c4f7765e - std::sys::unix::thread::Thread::new::thread_start::h984039c12b16d4d5
  37:     0x7f28c4d0eb43 - <unknown>
  38:     0x7f28c4da0a00 - <unknown>
  39:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (42f06293f 2023-04-01) running on x86_64-unknown-linux-gnu


note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z allow-features=binary-dep-depinfo,proc_macro_span,proc_macro_span_shrink,proc_macro_diagnostic,proc_macro_internals,proc_macro_diagnostic,proc_macro_span,proc_macro_span_shrink
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
