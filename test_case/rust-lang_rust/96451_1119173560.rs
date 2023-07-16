plain
   Compiling object v0.26.2
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.12.0
   Compiling addr2line v0.16.0
thread 'rustc' panicked at 'tried to get overflow intrinsic for op applied to non-int type', compiler/rustc_codegen_llvm/src/builder.rs:339:18
   0:     0x7f967ceeec52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   0:     0x7f967ceeec52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   1:     0x7f967cf56618 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f967cedf191 - std::io::Write::write_fmt::hf3faa85fa7d28190
   3:     0x7f967cef1f96 - std::panicking::default_hook::{{closure}}::h243e0a014f6b15da
   4:     0x7f967cef1b8d - std::panicking::default_hook::hdf681f01978f1e20
   5:     0x7f967da41591 - rustc_driver[6cf9c77043e56e2d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f967cef2930 - std::panicking::rust_panic_with_hook::h1c127668bc0f49d8
   7:     0x7f967cef2709 - std::panicking::begin_panic_handler::{{closure}}::hdc297c549f81c3b7
   8:     0x7f967ceef1f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h7b90b067d1e7c19a
   9:     0x7f967cef2439 - rust_begin_unwind
  10:     0x7f967cea60b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f967dd0f69e - <rustc_codegen_llvm[edd65bb384c8e6e5]::builder::Builder as rustc_codegen_ssa[de2aeeb76c412fc6]::traits::builder::BuilderMethods>::checked_binop
  12:     0x7f967dbf8521 - <rustc_codegen_ssa[de2aeeb76c412fc6]::mir::FunctionCx<rustc_codegen_llvm[edd65bb384c8e6e5]::builder::Builder>>::codegen_rvalue_operand
  13:     0x7f967dbff2a3 - <rustc_codegen_ssa[de2aeeb76c412fc6]::mir::FunctionCx<rustc_codegen_llvm[edd65bb384c8e6e5]::builder::Builder>>::codegen_block
  14:     0x7f967dbf4666 - rustc_codegen_ssa[de2aeeb76c412fc6]::mir::codegen_mir::<rustc_codegen_llvm[edd65bb384c8e6e5]::builder::Builder>
  15:     0x7f967dc8a346 - rustc_codegen_ssa[de2aeeb76c412fc6]::base::codegen_instance::<rustc_codegen_llvm[edd65bb384c8e6e5]::builder::Builder>
  16:     0x7f967dd00f39 - <rustc_middle[dc079adbce280f15]::mir::mono::MonoItem as rustc_codegen_ssa[de2aeeb76c412fc6]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[edd65bb384c8e6e5]::builder::Builder>
  17:     0x7f967dc76034 - rustc_codegen_llvm[edd65bb384c8e6e5]::base::compile_codegen_unit::module_codegen
  18:     0x7f967dc74797 - rustc_codegen_llvm[edd65bb384c8e6e5]::base::compile_codegen_unit
  19:     0x7f967dcb06fb - <rustc_codegen_llvm[edd65bb384c8e6e5]::LlvmCodegenBackend as rustc_codegen_ssa[de2aeeb76c412fc6]::traits::backend::CodegenBackend>::codegen_crate
  20:     0x7f967db5cda8 - <rustc_session[bb98eeba4fc964d8]::session::Session>::time::<alloc[24f448636cd10183]::boxed::Box<dyn core[10878fb91fc84a80]::any::Any>, rustc_interface[575397cbf047e88c]::passes::start_codegen::{closure#0}>
  21:     0x7f967db50f38 - <rustc_interface[575397cbf047e88c]::passes::QueryContext>::enter::<<rustc_interface[575397cbf047e88c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[10878fb91fc84a80]::result::Result<alloc[24f448636cd10183]::boxed::Box<dyn core[10878fb91fc84a80]::any::Any>, rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>
  22:     0x7f967db33a6e - <rustc_interface[575397cbf047e88c]::queries::Queries>::ongoing_codegen
  23:     0x7f967d9d7058 - <rustc_interface[575397cbf047e88c]::interface::Compiler>::enter::<rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[575397cbf047e88c]::queries::Linker>, rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>
  24:     0x7f967d9b9116 - rustc_span[188b5a9c74ea64c9]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_interface[575397cbf047e88c]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#1}>
  25:     0x7f967d9d849f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[188b5a9c74ea64c9]::SessionGlobals>>::set::<rustc_interface[575397cbf047e88c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>
  26:     0x7f967da2cf69 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[575397cbf047e88c]::util::run_in_thread_pool_with_globals<rustc_interface[575397cbf047e88c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>
  27:     0x7f967d9ebcd1 - std[38ff3720b7fd637]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[575397cbf047e88c]::util::run_in_thread_pool_with_globals<rustc_interface[575397cbf047e88c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  28:     0x7f967da2f1f2 - <<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[575397cbf047e88c]::util::run_in_thread_pool_with_globals<rustc_interface[575397cbf047e88c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>, rustc_driver[6cf9c77043e56e2d]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[12e84e64950518c8]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  29:     0x7f967ceff333 - std::sys::unix::thread::Thread::new::thread_start::h38902d511e7013ce
  30:     0x7f967744f609 - start_thread
  31:     0x7f967cd62163 - clone
  32:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (9e23ac0ce 2022-05-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
