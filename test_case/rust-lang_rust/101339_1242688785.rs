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
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
[RUSTC-TIMING] rustc_middle test:false 124.567
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
[RUSTC-TIMING] rustc_privacy test:false 12.898
thread 'rustc' panicked at 'assertion failed: target_offset >= offset', compiler/rustc_codegen_llvm/src/type_of.rs:129:9
   0:     0x7f2a86ee0efd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::heec03eb9b2106f99
   0:     0x7f2a86ee0efd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::heec03eb9b2106f99
   1:     0x7f2a86f46f79 - core::fmt::write::hdc2190284bc6d0d4
   2:     0x7f2a86ed1e41 - std::io::Write::write_fmt::h42741f15affd0ecb
   3:     0x7f2a86ee3f0e - std::panicking::default_hook::{{closure}}::ha3e47422d6cfd2aa
   4:     0x7f2a86ee3bd7 - std::panicking::default_hook::h06e3da70e0c4e264
   5:     0x7f2a878d5a74 - rustc_driver[3636f0421e6d8c75]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2a86ee46b7 - std::panicking::rust_panic_with_hook::h332b5ce1ed540d19
   7:     0x7f2a86ee44aa - std::panicking::begin_panic_handler::{{closure}}::hf2b83e53a3e49fbf
   8:     0x7f2a86ee1464 - std::sys_common::backtrace::__rust_end_short_backtrace::hc87a83fccecce6d2
   9:     0x7f2a86ee41a2 - rust_begin_unwind
  10:     0x7f2a86e97c23 - core::panicking::panic_fmt::h55dfedac6f6597d4
  11:     0x7f2a86e97af3 - core::panicking::panic::h858d0a26258d295c
  12:     0x7f2a87b12d83 - rustc_codegen_llvm[7b6a0d1032d5a3f0]::type_of::struct_llfields
  13:     0x7f2a87afbb65 - <rustc_target[8d82747607a543bd]::abi::TyAndLayout<rustc_middle[10b8fe355026857d]::ty::Ty> as rustc_codegen_llvm[7b6a0d1032d5a3f0]::type_of::LayoutLlvmExt>::llvm_type
  14:     0x7f2a87b008ba - <rustc_codegen_ssa[ace718b546837800]::mir::place::PlaceRef<&rustc_codegen_llvm[7b6a0d1032d5a3f0]::llvm_::ffi::Value>>::project_downcast::<rustc_codegen_llvm[7b6a0d1032d5a3f0]::builder::Builder>
  15:     0x7f2a87c31507 - <rustc_codegen_ssa[ace718b546837800]::mir::FunctionCx<rustc_codegen_llvm[7b6a0d1032d5a3f0]::builder::Builder>>::codegen_place
  16:     0x7f2a87c22b7d - <rustc_codegen_ssa[ace718b546837800]::mir::FunctionCx<rustc_codegen_llvm[7b6a0d1032d5a3f0]::builder::Builder>>::codegen_rvalue_operand
  17:     0x7f2a87c2974a - <rustc_codegen_ssa[ace718b546837800]::mir::FunctionCx<rustc_codegen_llvm[7b6a0d1032d5a3f0]::builder::Builder>>::codegen_block
  18:     0x7f2a87c1e446 - rustc_codegen_ssa[ace718b546837800]::mir::codegen_mir::<rustc_codegen_llvm[7b6a0d1032d5a3f0]::builder::Builder>
  19:     0x7f2a87b32937 - rustc_codegen_ssa[ace718b546837800]::base::codegen_instance::<rustc_codegen_llvm[7b6a0d1032d5a3f0]::builder::Builder>
  20:     0x7f2a87bac694 - rustc_codegen_llvm[7b6a0d1032d5a3f0]::base::compile_codegen_unit::module_codegen
  21:     0x7f2a87adf407 - <rustc_query_system[9c9e75e4d6a48ae]::dep_graph::graph::DepGraph<rustc_middle[10b8fe355026857d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[10b8fe355026857d]::ty::context::TyCtxt, rustc_span[730d915af92419e1]::symbol::Symbol, rustc_codegen_ssa[ace718b546837800]::ModuleCodegen<rustc_codegen_llvm[7b6a0d1032d5a3f0]::ModuleLlvm>>
  22:     0x7f2a87bac231 - rustc_codegen_llvm[7b6a0d1032d5a3f0]::base::compile_codegen_unit
  23:     0x7f2a87b31eaf - rustc_codegen_ssa[ace718b546837800]::base::codegen_crate::<rustc_codegen_llvm[7b6a0d1032d5a3f0]::LlvmCodegenBackend>
  24:     0x7f2a87bf3aca - <rustc_codegen_llvm[7b6a0d1032d5a3f0]::LlvmCodegenBackend as rustc_codegen_ssa[ace718b546837800]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7f2a87a65cab - <rustc_session[d30048c4c30c61e5]::session::Session>::time::<alloc[45623a189840f9f9]::boxed::Box<dyn core[c1e30f1bd259d119]::any::Any>, rustc_interface[e5898ec9738d6bfc]::passes::start_codegen::{closure#0}>
  26:     0x7f2a87a1bf2d - <rustc_interface[e5898ec9738d6bfc]::passes::QueryContext>::enter::<<rustc_interface[e5898ec9738d6bfc]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c1e30f1bd259d119]::result::Result<alloc[45623a189840f9f9]::boxed::Box<dyn core[c1e30f1bd259d119]::any::Any>, rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>
  27:     0x7f2a87a090ba - <rustc_interface[e5898ec9738d6bfc]::queries::Queries>::ongoing_codegen
  28:     0x7f2a878d7657 - <rustc_interface[e5898ec9738d6bfc]::interface::Compiler>::enter::<rustc_driver[3636f0421e6d8c75]::run_compiler::{closure#1}::{closure#2}, core[c1e30f1bd259d119]::result::Result<core[c1e30f1bd259d119]::option::Option<rustc_interface[e5898ec9738d6bfc]::queries::Linker>, rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>
  29:     0x7f2a878c4466 - rustc_span[730d915af92419e1]::with_source_map::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>, rustc_interface[e5898ec9738d6bfc]::interface::create_compiler_and_run<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>, rustc_driver[3636f0421e6d8c75]::run_compiler::{closure#1}>::{closure#1}>
  30:     0x7f2a878eb587 - rustc_interface[e5898ec9738d6bfc]::interface::create_compiler_and_run::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>, rustc_driver[3636f0421e6d8c75]::run_compiler::{closure#1}>
  31:     0x7f2a878bb592 - <scoped_tls[5efd78fa53ce51fd]::ScopedKey<rustc_span[730d915af92419e1]::SessionGlobals>>::set::<rustc_interface[e5898ec9738d6bfc]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>, rustc_driver[3636f0421e6d8c75]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>
  32:     0x7f2a8793773f - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e5898ec9738d6bfc]::util::run_in_thread_pool_with_globals<rustc_interface[e5898ec9738d6bfc]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>, rustc_driver[3636f0421e6d8c75]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>
  33:     0x7f2a878c5b9e - std[8a3c335779a4ef7b]::panic::catch_unwind::<core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[e5898ec9738d6bfc]::util::run_in_thread_pool_with_globals<rustc_interface[e5898ec9738d6bfc]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>, rustc_driver[3636f0421e6d8c75]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>
  34:     0x7f2a87938f20 - <<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[e5898ec9738d6bfc]::util::run_in_thread_pool_with_globals<rustc_interface[e5898ec9738d6bfc]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>, rustc_driver[3636f0421e6d8c75]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[15b461d5cc289186]::ErrorGuaranteed>>::{closure#1} as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f2a86ef06f5 - std::sys::unix::thread::Thread::new::thread_start::h4ca20864c1f05a1d
  36:     0x7f2a86c90b43 - <unknown>
  37:     0x7f2a86d22a00 - <unknown>
  38:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (761312563 2022-09-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z randomize-layout -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
