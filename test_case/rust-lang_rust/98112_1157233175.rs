plain
Successfully built 80956fd0f57a
Successfully tagged rust-ci:latest
Built container sha256:80956fd0f57a8f71d00c871e78139a59a762bc663fcc8e4610d69bbbc952ace2
Uploading finished image to https://ci-caches.rust-lang.org/docker/6a9f2d15c383e5a8c3f65ed87cf17e97fb6e5a5a5df550e82f2c9b9f0bc64573d60ff8ccfa7fbd06fb3e3d891dab927e307eab2dcff6cc2e2c3d2993fe31d69b
upload failed: - to s3://rust-lang-ci-sccache2/docker/6a9f2d15c383e5a8c3f65ed87cf17e97fb6e5a5a5df550e82f2c9b9f0bc64573d60ff8ccfa7fbd06fb3e3d891dab927e307eab2dcff6cc2e2c3d2993fe31d69b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
   Compiling itertools v0.10.1
   Compiling miniz_oxide v0.4.0
   Compiling unicode-normalization v0.1.13
   Compiling unic-emoji-char v0.9.0
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:116:18: not immediate: OperandRef(Pair((%"alloc::sync::ArcInner<dyn subscriber::Subscriber + core::marker::Send + core::marker::Sync>"*:  %21 = extractvalue { %"alloc::sync::ArcInner<dyn subscriber::Subscriber + core::marker::Send + core::marker::Sync>"*, [3 x i64]* } %20, 0), ([3 x i64]*:  %22 = extractvalue { %"alloc::sync::ArcInner<dyn subscriber::Subscriber + core::marker::Send + core::marker::Sync>"*, [3 x i64]* } %20, 1)) @ TyAndLayout { ty: *mut alloc::sync::ArcInner<dyn subscriber::Subscriber + std::marker::Send + std::marker::Sync>, layout: Layout { fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], memory_index: [0, 1] }, variants: Single { index: 0 }, abi: ScalarPair(Initialized { value: Pointer, valid_range: 0..=18446744073709551615 }, Initialized { value: Pointer, valid_range: 1..=18446744073709551615 }), largest_niche: Some(Niche { offset: Size(8 bytes), value: Pointer, valid_range: 1..=18446744073709551615 }), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, size: Size(16 bytes) } })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1362:9
stack backtrace:
stack backtrace:
   0:     0x7f23a5023ee2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7f23a508bc08 - core::fmt::write::ha01458c252ca8e28
   2:     0x7f23a5014191 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7f23a5027219 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7f23a5026eba - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7f23a5ae0191 - rustc_driver[4ab23b96e64fefcb]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f23a5027a7f - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7f23a8487483 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[2831149b0503dd2]::ExplicitBug>::{closure#0}
   8:     0x7f23a84815f6 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[2831149b0503dd2]::ExplicitBug>::{closure#0}, !>
   9:     0x7f23a5a67926 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[2831149b0503dd2]::ExplicitBug>
  10:     0x7f23a83b3b06 - std[836a811975e52724]::panic::panic_any::<rustc_errors[2831149b0503dd2]::ExplicitBug>
  11:     0x7f23a83b18d6 - <rustc_errors[2831149b0503dd2]::HandlerInner>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  12:     0x7f23a83b1588 - <rustc_errors[2831149b0503dd2]::Handler>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  13:     0x7f23a849a94e - rustc_middle[e2b5d180485462e]::ty::context::tls::with_opt::<rustc_middle[e2b5d180485462e]::util::bug::opt_span_bug_fmt<rustc_span[3917deda49487040]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f23a849a9b9 - rustc_middle[e2b5d180485462e]::util::bug::opt_span_bug_fmt::<rustc_span[3917deda49487040]::span_encoding::Span>
  15:     0x7f23a5a6d0f5 - rustc_middle[e2b5d180485462e]::util::bug::bug_fmt
  16:     0x7f23a5e28536 - <rustc_codegen_ssa[42d1a03b19b246a8]::mir::operand::OperandRef<&rustc_codegen_llvm[d5a44b14496e869c]::llvm_::ffi::Value>>::immediate
  17:     0x7f23a5e3f8a6 - <rustc_codegen_ssa[42d1a03b19b246a8]::mir::FunctionCx<rustc_codegen_llvm[d5a44b14496e869c]::builder::Builder>>::codegen_rvalue_operand
  18:     0x7f23a5e466f0 - <rustc_codegen_ssa[42d1a03b19b246a8]::mir::FunctionCx<rustc_codegen_llvm[d5a44b14496e869c]::builder::Builder>>::codegen_block
  19:     0x7f23a5e3af86 - rustc_codegen_ssa[42d1a03b19b246a8]::mir::codegen_mir::<rustc_codegen_llvm[d5a44b14496e869c]::builder::Builder>
  20:     0x7f23a5d73026 - rustc_codegen_ssa[42d1a03b19b246a8]::base::codegen_instance::<rustc_codegen_llvm[d5a44b14496e869c]::builder::Builder>
  21:     0x7f23a5e0ab06 - <rustc_middle[e2b5d180485462e]::mir::mono::MonoItem as rustc_codegen_ssa[42d1a03b19b246a8]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[d5a44b14496e869c]::builder::Builder>
  22:     0x7f23a5d61564 - rustc_codegen_llvm[d5a44b14496e869c]::base::compile_codegen_unit::module_codegen
  23:     0x7f23a5d5fc3a - rustc_codegen_llvm[d5a44b14496e869c]::base::compile_codegen_unit
  24:     0x7f23a5d92a64 - <rustc_codegen_llvm[d5a44b14496e869c]::LlvmCodegenBackend as rustc_codegen_ssa[42d1a03b19b246a8]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7f23a5c16308 - <rustc_session[c6fdfe7fd0a3b7ea]::session::Session>::time::<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_interface[64ff3d3fe0dd6459]::passes::start_codegen::{closure#0}>
  26:     0x7f23a5bfde66 - <rustc_interface[64ff3d3fe0dd6459]::passes::QueryContext>::enter::<<rustc_interface[64ff3d3fe0dd6459]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6d9550a4e960c99f]::result::Result<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_errors[2831149b0503dd2]::ErrorGuaranteed>>
  27:     0x7f23a5ca0228 - <rustc_interface[64ff3d3fe0dd6459]::queries::Queries>::ongoing_codegen
  28:     0x7f23a5afc648 - <rustc_interface[64ff3d3fe0dd6459]::interface::Compiler>::enter::<rustc_driver[4ab23b96e64fefcb]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[64ff3d3fe0dd6459]::queries::Linker>, rustc_errors[2831149b0503dd2]::ErrorGuaranteed>>
  29:     0x7f23a5b59206 - rustc_span[3917deda49487040]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>, rustc_interface[64ff3d3fe0dd6459]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>, rustc_driver[4ab23b96e64fefcb]::run_compiler::{closure#1}>::{closure#1}>
  30:     0x7f23a5afda4e - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[3917deda49487040]::SessionGlobals>>::set::<rustc_interface[64ff3d3fe0dd6459]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>, rustc_driver[4ab23b96e64fefcb]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>>
  31:     0x7f23a5b5e2df - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[64ff3d3fe0dd6459]::util::run_in_thread_pool_with_globals<rustc_interface[64ff3d3fe0dd6459]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>, rustc_driver[4ab23b96e64fefcb]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>>
  32:     0x7f23a5b514f1 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[64ff3d3fe0dd6459]::util::run_in_thread_pool_with_globals<rustc_interface[64ff3d3fe0dd6459]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>, rustc_driver[4ab23b96e64fefcb]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[2831149b0503dd2]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7f23a50344c3 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  34:     0x7f239f584609 - start_thread
  35:     0x7f23a4e97133 - clone
  36:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (0fb805e8a 2022-06-16) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
