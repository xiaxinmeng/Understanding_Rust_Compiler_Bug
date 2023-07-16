plain
 ---> 232a30d9f30c
Step 7/9 : ENV NO_DOWNLOAD_CI_LLVM 1
 ---> Using cache
 ---> dd9cad906204
Step 8/9 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-13       --enable-llvm-link-shared       --set rust.randomize-layout=true       --set rust.thin-lto-import-instr-limit=10
 ---> c09b4f148c35
Step 9/9 : ENV SCRIPT ../x.py --stage 2 test --exclude src/tools/tidy &&            ../x --stage 2 test src/test/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            ../x.ps1 --stage 2 test src/test/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 2 test src/tools/tidy
 ---> Using cache
 ---> bdd471ba91b1
---
configure: 
configure: build.build          := x86_64-unknown-linux-gnu
configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-13/bin/ll ...
configure: llvm.link-shared     := True
configure: rust.randomize-layout := True
configure: build.print-step-timings := True
configure: build.metrics        := True
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
---
[RUSTC-TIMING] rustc_lint test:false 45.566
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
[RUSTC-TIMING] rustc_privacy test:false 13.718
[RUSTC-TIMING] rustc_middle test:false 128.352
thread 'rustc' panicked at 'assertion failed: target_offset >= offset', compiler/rustc_codegen_llvm/src/type_of.rs:129:9
   0:     0x7f20e7d016b0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h189266b990bc4cfa
   1:     0x7f20e7d6c1b9 - core::fmt::write::hdc2190284bc6d0d4
   1:     0x7f20e7d6c1b9 - core::fmt::write::hdc2190284bc6d0d4
   2:     0x7f20e7cf1b41 - std::io::Write::write_fmt::hcd3f9e3061f38481
   3:     0x7f20e7d04a8e - std::panicking::default_hook::{{closure}}::hc35d6a83060811fc
   4:     0x7f20e7d04751 - std::panicking::default_hook::h4025000d4f88908e
   5:     0x7f20e86fda64 - rustc_driver[5a9a929c23ac41f2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f20e7d0525f - std::panicking::rust_panic_with_hook::hef0d594d8cdfe6a9
   7:     0x7f20e7d0504a - std::panicking::begin_panic_handler::{{closure}}::h2eca50bd66c97083
   8:     0x7f20e7d01c74 - std::sys_common::backtrace::__rust_end_short_backtrace::hbbeb6c379416c254
   9:     0x7f20e7d04d22 - rust_begin_unwind
  10:     0x7f20e7cb5073 - core::panicking::panic_fmt::h55dfedac6f6597d4
  11:     0x7f20e7cb4f43 - core::panicking::panic::h858d0a26258d295c
  12:     0x7f20e89d4fbf - rustc_codegen_llvm[a25c3b705918d870]::type_of::struct_llfields
  13:     0x7f20e899ceff - <rustc_target[911f7b142972ffe9]::abi::TyAndLayout<rustc_middle[30138fff79654cb0]::ty::Ty> as rustc_codegen_llvm[a25c3b705918d870]::type_of::LayoutLlvmExt>::llvm_type
  14:     0x7f20e89a1d6b - <rustc_codegen_ssa[9115261bc2c1df99]::mir::place::PlaceRef<&rustc_codegen_llvm[a25c3b705918d870]::llvm_::ffi::Value>>::project_downcast::<rustc_codegen_llvm[a25c3b705918d870]::builder::Builder>
  15:     0x7f20e8aa2b9c - <rustc_codegen_ssa[9115261bc2c1df99]::mir::FunctionCx<rustc_codegen_llvm[a25c3b705918d870]::builder::Builder>>::codegen_place
  16:     0x7f20e8a977be - <rustc_codegen_ssa[9115261bc2c1df99]::mir::FunctionCx<rustc_codegen_llvm[a25c3b705918d870]::builder::Builder>>::codegen_rvalue_operand
  17:     0x7f20e8a913ea - rustc_codegen_ssa[9115261bc2c1df99]::mir::codegen_mir::<rustc_codegen_llvm[a25c3b705918d870]::builder::Builder>
  18:     0x7f20e89142e7 - rustc_codegen_ssa[9115261bc2c1df99]::base::codegen_instance::<rustc_codegen_llvm[a25c3b705918d870]::builder::Builder>
  19:     0x7f20e8a263d9 - <rustc_middle[30138fff79654cb0]::mir::mono::MonoItem as rustc_codegen_ssa[9115261bc2c1df99]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[a25c3b705918d870]::builder::Builder>
  20:     0x7f20e8a6608f - rustc_codegen_llvm[a25c3b705918d870]::base::compile_codegen_unit::module_codegen
  21:     0x7f20e897b89d - <rustc_query_system[5d45b85c8959cba6]::dep_graph::graph::DepGraph<rustc_middle[30138fff79654cb0]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[30138fff79654cb0]::ty::context::TyCtxt, rustc_span[4fe8128055cb8f86]::symbol::Symbol, rustc_codegen_ssa[9115261bc2c1df99]::ModuleCodegen<rustc_codegen_llvm[a25c3b705918d870]::ModuleLlvm>>
  22:     0x7f20e8a65cc1 - rustc_codegen_llvm[a25c3b705918d870]::base::compile_codegen_unit
  23:     0x7f20e891342e - rustc_codegen_ssa[9115261bc2c1df99]::base::codegen_crate::<rustc_codegen_llvm[a25c3b705918d870]::LlvmCodegenBackend>
  24:     0x7f20e894447a - <rustc_codegen_llvm[a25c3b705918d870]::LlvmCodegenBackend as rustc_codegen_ssa[9115261bc2c1df99]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7f20e886b3b6 - <rustc_interface[a43ed275f0382cfd]::passes::QueryContext>::enter::<<rustc_interface[a43ed275f0382cfd]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c1e30f1bd259d119]::result::Result<alloc[45623a189840f9f9]::boxed::Box<dyn core[c1e30f1bd259d119]::any::Any>, rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>
  26:     0x7f20e88422ce - <rustc_interface[a43ed275f0382cfd]::queries::Queries>::ongoing_codegen
  27:     0x7f20e86ff9a0 - <rustc_interface[a43ed275f0382cfd]::interface::Compiler>::enter::<rustc_driver[5a9a929c23ac41f2]::run_compiler::{closure#1}::{closure#2}, core[c1e30f1bd259d119]::result::Result<core[c1e30f1bd259d119]::option::Option<rustc_interface[a43ed275f0382cfd]::queries::Linker>, rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>
  28:     0x7f20e86e8a08 - rustc_span[4fe8128055cb8f86]::with_source_map::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>, rustc_interface[a43ed275f0382cfd]::interface::create_compiler_and_run<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>, rustc_driver[5a9a929c23ac41f2]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7f20e8700dcd - rustc_interface[a43ed275f0382cfd]::interface::create_compiler_and_run::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>, rustc_driver[5a9a929c23ac41f2]::run_compiler::{closure#1}>
  30:     0x7f20e87832c2 - <scoped_tls[5efd78fa53ce51fd]::ScopedKey<rustc_span[4fe8128055cb8f86]::SessionGlobals>>::set::<rustc_interface[a43ed275f0382cfd]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>, rustc_driver[5a9a929c23ac41f2]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>
  31:     0x7f20e876d3ff - std[b19370fede7caea]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a43ed275f0382cfd]::util::run_in_thread_pool_with_globals<rustc_interface[a43ed275f0382cfd]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>, rustc_driver[5a9a929c23ac41f2]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>
  32:     0x7f20e86ea2fe - std[b19370fede7caea]::panicking::try::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>, core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<<std[b19370fede7caea]::thread::Builder>::spawn_unchecked_<rustc_interface[a43ed275f0382cfd]::util::run_in_thread_pool_with_globals<rustc_interface[a43ed275f0382cfd]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>, rustc_driver[5a9a929c23ac41f2]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7f20e876f510 - <<std[b19370fede7caea]::thread::Builder>::spawn_unchecked_<rustc_interface[a43ed275f0382cfd]::util::run_in_thread_pool_with_globals<rustc_interface[a43ed275f0382cfd]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>, rustc_driver[5a9a929c23ac41f2]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[f550ba839837ab93]::ErrorGuaranteed>>::{closure#1} as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f20e7d113c5 - std::sys::unix::thread::Thread::new::thread_start::h8972e60baefc7a3d
  35:     0x7f20e7aacb43 - <unknown>
  36:     0x7f20e7b3ea00 - <unknown>
  37:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (34e35a95c 2022-09-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z randomize-layout -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
