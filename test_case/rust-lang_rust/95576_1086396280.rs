plain
---- [ui] ui/mir/ssa-analysis-regression-50041.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: i < this.fields.count()', /checkout/compiler/rustc_middle/src/ty/layout.rs:2326:21
stack backtrace:
   0:     0x7fe51fbcd16c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb9b5be993b3728c0
   1:     0x7fe51fc3356f - core::fmt::write::h5d8882aa143ccea7
   2:     0x7fe51fbbca11 - std::io::Write::write_fmt::h40daf24893c2e925
   3:     0x7fe51fbccf8b - std::sys_common::backtrace::print::h58300a761e0d5fe1
   4:     0x7fe51fbd09a4 - std::panicking::default_hook::{{closure}}::h4a64a0d920f8f5da
   5:     0x7fe51fbd055d - std::panicking::default_hook::h4638191b4a20340e
   6:     0x7fe520661a41 - rustc_driver[e94838e84d582fbe]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe51fbd108b - std::panicking::rust_panic_with_hook::haf462d19f4b28ba1
   8:     0x7fe51fbd0e79 - std::panicking::begin_panic_handler::{{closure}}::h40175c4e98cc81a0
   9:     0x7fe51fbcd684 - std::sys_common::backtrace::__rust_end_short_backtrace::hec3c55b8cd1d8a9f
  10:     0x7fe51fbd0b99 - rust_begin_unwind
  11:     0x7fe51fb7fca3 - core::panicking::panic_fmt::h3addf34388de9cbb
  12:     0x7fe51fb7fb6d - core::panicking::panic::h9defd9d978323742
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  13:     0x7fe52088050f - <rustc_middle[8b5248ac413bee95]::ty::Ty as rustc_target[74ab3b8badd6d81b]::abi::TyAbiInterface<_>>::ty_and_layout_field::field_ty_or_layout::<rustc_codegen_llvm[b3f5220b536618e5]::context::CodegenCx>
  14:     0x7fe520901575 - <rustc_middle[8b5248ac413bee95]::ty::Ty as rustc_target[74ab3b8badd6d81b]::abi::TyAbiInterface<rustc_codegen_llvm[b3f5220b536618e5]::context::CodegenCx>>::ty_and_layout_field
  15:     0x7fe5209d517d - <rustc_codegen_ssa[69e5a2d973fe6906]::mir::operand::OperandRef<&rustc_codegen_llvm[b3f5220b536618e5]::llvm_::ffi::Value>>::extract_field::<rustc_codegen_llvm[b3f5220b536618e5]::builder::Builder>
  16:     0x7fe5208ae49e - <rustc_codegen_ssa[69e5a2d973fe6906]::mir::FunctionCx<rustc_codegen_llvm[b3f5220b536618e5]::builder::Builder>>::codegen_consume
  17:     0x7fe5208aea78 - <rustc_codegen_ssa[69e5a2d973fe6906]::mir::FunctionCx<rustc_codegen_llvm[b3f5220b536618e5]::builder::Builder>>::codegen_operand
  18:     0x7fe52089c217 - <rustc_codegen_ssa[69e5a2d973fe6906]::mir::FunctionCx<rustc_codegen_llvm[b3f5220b536618e5]::builder::Builder>>::codegen_rvalue
  19:     0x7fe5208a5817 - <rustc_codegen_ssa[69e5a2d973fe6906]::mir::FunctionCx<rustc_codegen_llvm[b3f5220b536618e5]::builder::Builder>>::codegen_block
  20:     0x7fe520899326 - rustc_codegen_ssa[69e5a2d973fe6906]::mir::codegen_mir::<rustc_codegen_llvm[b3f5220b536618e5]::builder::Builder>
  21:     0x7fe5209418f6 - rustc_codegen_ssa[69e5a2d973fe6906]::base::codegen_instance::<rustc_codegen_llvm[b3f5220b536618e5]::builder::Builder>
  22:     0x7fe5209be9c4 - <rustc_middle[8b5248ac413bee95]::mir::mono::MonoItem as rustc_codegen_ssa[69e5a2d973fe6906]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[b3f5220b536618e5]::builder::Builder>
  23:     0x7fe520934784 - rustc_codegen_llvm[b3f5220b536618e5]::base::compile_codegen_unit::module_codegen
  24:     0x7fe520932a6a - rustc_codegen_llvm[b3f5220b536618e5]::base::compile_codegen_unit
  25:     0x7fe5209404b6 - rustc_codegen_ssa[69e5a2d973fe6906]::base::codegen_crate::<rustc_codegen_llvm[b3f5220b536618e5]::LlvmCodegenBackend>
  26:     0x7fe52097e6e9 - <rustc_codegen_llvm[b3f5220b536618e5]::LlvmCodegenBackend as rustc_codegen_ssa[69e5a2d973fe6906]::traits::backend::CodegenBackend>::codegen_crate
  27:     0x7fe5207b5d81 - <rustc_session[e53d8835b47fd942]::session::Session>::time::<alloc[39129a5dc73cf362]::boxed::Box<dyn core[69fa6c144d448b89]::any::Any>, rustc_interface[40ce63f0f6188d42]::passes::start_codegen::{closure#0}>
  28:     0x7fe52086419e - <rustc_interface[40ce63f0f6188d42]::passes::QueryContext>::enter::<<rustc_interface[40ce63f0f6188d42]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[69fa6c144d448b89]::result::Result<alloc[39129a5dc73cf362]::boxed::Box<dyn core[69fa6c144d448b89]::any::Any>, rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>
  29:     0x7fe5207fcb0e - <rustc_interface[40ce63f0f6188d42]::queries::Queries>::ongoing_codegen
  30:     0x7fe5206c0788 - <rustc_interface[40ce63f0f6188d42]::interface::Compiler>::enter::<rustc_driver[e94838e84d582fbe]::run_compiler::{closure#1}::{closure#2}, core[69fa6c144d448b89]::result::Result<core[69fa6c144d448b89]::option::Option<rustc_interface[40ce63f0f6188d42]::queries::Linker>, rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>
  31:     0x7fe5206d5a7b - rustc_span[1a0c247fd6ef9c32]::with_source_map::<core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>, rustc_interface[40ce63f0f6188d42]::interface::create_compiler_and_run<core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>, rustc_driver[e94838e84d582fbe]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7fe5206c13b3 - <scoped_tls[f372c4f9966ba259]::ScopedKey<rustc_span[1a0c247fd6ef9c32]::SessionGlobals>>::set::<rustc_interface[40ce63f0f6188d42]::interface::run_compiler<core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>, rustc_driver[e94838e84d582fbe]::run_compiler::{closure#1}>::{closure#0}, core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>
  33:     0x7fe520688b39 - std[634e8db096c2ed47]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[40ce63f0f6188d42]::util::run_in_thread_pool_with_globals<rustc_interface[40ce63f0f6188d42]::interface::run_compiler<core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>, rustc_driver[e94838e84d582fbe]::run_compiler::{closure#1}>::{closure#0}, core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>::{closure#0}, core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>
  34:     0x7fe5206e89f1 - std[634e8db096c2ed47]::panicking::try::<core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>, core[69fa6c144d448b89]::panic::unwind_safe::AssertUnwindSafe<<std[634e8db096c2ed47]::thread::Builder>::spawn_unchecked_<rustc_interface[40ce63f0f6188d42]::util::run_in_thread_pool_with_globals<rustc_interface[40ce63f0f6188d42]::interface::run_compiler<core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>, rustc_driver[e94838e84d582fbe]::run_compiler::{closure#1}>::{closure#0}, core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>::{closure#0}, core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7fe5206df873 - <<std[634e8db096c2ed47]::thread::Builder>::spawn_unchecked_<rustc_interface[40ce63f0f6188d42]::util::run_in_thread_pool_with_globals<rustc_interface[40ce63f0f6188d42]::interface::run_compiler<core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>, rustc_driver[e94838e84d582fbe]::run_compiler::{closure#1}>::{closure#0}, core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>::{closure#0}, core[69fa6c144d448b89]::result::Result<(), rustc_errors[b6b4c1140a2ec3eb]::ErrorGuaranteed>>::{closure#1} as core[69fa6c144d448b89]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fe51fbdc903 - std::sys::unix::thread::Thread::new::thread_start::hebf04978f304d5ec
  37:     0x7fe51a12b609 - start_thread
  38:     0x7fe51fa3e163 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (da25bb6e7 2022-04-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=4
query stack during panic:
end of query stack
------------------------------------------

