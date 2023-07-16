plain
   Compiling object v0.30.1
   Compiling miniz_oxide v0.6.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.19.0
error: internal compiler error: /checkout/compiler/rustc_middle/src/ty/consts/int.rs:223:13: expected int of size 4, but got size 8
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1643:9
stack backtrace:
   0:     0x7f489e0db444 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h59466227deb404f8
   1:     0x7f489e1428d8 - core::fmt::write::hfb297b11d766670e
   1:     0x7f489e1428d8 - core::fmt::write::hfb297b11d766670e
   2:     0x7f489e0cfb71 - std::io::Write::write_fmt::he039725b8229e03e
   3:     0x7f489e0db251 - std::sys_common::backtrace::print::h5e713a66018f6180
   4:     0x7f489e0de31a - std::panicking::default_hook::{{closure}}::hea3b943d1520e9be
   5:     0x7f489e0ddffc - std::panicking::default_hook::h4a2c85c0cedffb77
   6:     0x7f489eb9e805 - rustc_driver_impl[8286e531aa15858]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f489e0dea27 - std::panicking::rust_panic_with_hook::h6aa99442f9c3b24d
   8:     0x7f48a185ede3 - std[267aca1effb3fcd0]::panicking::begin_panic::<rustc_errors[d6cf9c723b80e0]::ExplicitBug>::{closure#0}
   9:     0x7f48a1857c06 - std[267aca1effb3fcd0]::sys_common::backtrace::__rust_end_short_backtrace::<std[267aca1effb3fcd0]::panicking::begin_panic<rustc_errors[d6cf9c723b80e0]::ExplicitBug>::{closure#0}, !>
  10:     0x7f489eb39796 - std[267aca1effb3fcd0]::panicking::begin_panic::<rustc_errors[d6cf9c723b80e0]::ExplicitBug>
  11:     0x7f48a181b277 - <rustc_errors[d6cf9c723b80e0]::HandlerInner>::bug::<&alloc[8d702a2c5c209e06]::string::String>
  12:     0x7f48a181a57d - <rustc_errors[d6cf9c723b80e0]::Handler>::bug::<&alloc[8d702a2c5c209e06]::string::String>
  13:     0x7f48a19c748c - rustc_middle[2ec8f3ddbf269a3f]::util::bug::opt_span_bug_fmt::<rustc_span[8dcf7f7285d48a69]::span_encoding::Span>::{closure#0}
  14:     0x7f48a19c63dc - rustc_middle[2ec8f3ddbf269a3f]::ty::context::tls::with_opt::<rustc_middle[2ec8f3ddbf269a3f]::util::bug::opt_span_bug_fmt<rustc_span[8dcf7f7285d48a69]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f48a19c6364 - rustc_middle[2ec8f3ddbf269a3f]::ty::context::tls::with_context_opt::<rustc_middle[2ec8f3ddbf269a3f]::ty::context::tls::with_opt<rustc_middle[2ec8f3ddbf269a3f]::util::bug::opt_span_bug_fmt<rustc_span[8dcf7f7285d48a69]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f489eb41632 - rustc_middle[2ec8f3ddbf269a3f]::util::bug::bug_fmt
  17:     0x7f489ee34a7f - <rustc_codegen_llvm[d481d0cdb9131f2]::context::CodegenCx as rustc_codegen_ssa[c2cd3ae65ab7c21]::traits::consts::ConstMethods>::scalar_to_backend
  18:     0x7f489edac7aa - <rustc_codegen_ssa[c2cd3ae65ab7c21]::mir::operand::OperandRef<&rustc_codegen_llvm[d481d0cdb9131f2]::llvm_::ffi::Value>>::from_const::<rustc_codegen_llvm[d481d0cdb9131f2]::builder::Builder>
  19:     0x7f489ee9e573 - <rustc_codegen_ssa[c2cd3ae65ab7c21]::mir::FunctionCx<rustc_codegen_llvm[d481d0cdb9131f2]::builder::Builder>>::codegen_operand
  20:     0x7f489ee97c31 - <rustc_codegen_ssa[c2cd3ae65ab7c21]::mir::FunctionCx<rustc_codegen_llvm[d481d0cdb9131f2]::builder::Builder>>::codegen_terminator
  21:     0x7f489ee88158 - rustc_codegen_ssa[c2cd3ae65ab7c21]::mir::codegen_mir::<rustc_codegen_llvm[d481d0cdb9131f2]::builder::Builder>
  22:     0x7f489ee41bfd - rustc_codegen_ssa[c2cd3ae65ab7c21]::base::codegen_instance::<rustc_codegen_llvm[d481d0cdb9131f2]::builder::Builder>
  23:     0x7f489ee70f08 - <rustc_middle[2ec8f3ddbf269a3f]::mir::mono::MonoItem as rustc_codegen_ssa[c2cd3ae65ab7c21]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[d481d0cdb9131f2]::builder::Builder>
  24:     0x7f489ed6eceb - rustc_codegen_llvm[d481d0cdb9131f2]::base::compile_codegen_unit::module_codegen
  25:     0x7f489ed6e1c7 - rustc_codegen_llvm[d481d0cdb9131f2]::base::compile_codegen_unit
  26:     0x7f489ee40ffa - rustc_codegen_ssa[c2cd3ae65ab7c21]::base::codegen_crate::<rustc_codegen_llvm[d481d0cdb9131f2]::LlvmCodegenBackend>
  27:     0x7f489ee14f30 - <rustc_codegen_llvm[d481d0cdb9131f2]::LlvmCodegenBackend as rustc_codegen_ssa[c2cd3ae65ab7c21]::traits::backend::CodegenBackend>::codegen_crate
  28:     0x7f489ec8d20f - rustc_interface[7a55d0aec9a4f803]::passes::start_codegen
  29:     0x7f489ecad666 - <rustc_middle[2ec8f3ddbf269a3f]::ty::context::GlobalCtxt>::enter::<<rustc_interface[7a55d0aec9a4f803]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[8433f9a650d1cc7a]::result::Result<alloc[8d702a2c5c209e06]::boxed::Box<dyn core[8433f9a650d1cc7a]::any::Any>, rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>
  30:     0x7f489ec90fd4 - <rustc_interface[7a55d0aec9a4f803]::queries::Queries>::ongoing_codegen
  31:     0x7f489ebe9bee - <rustc_interface[7a55d0aec9a4f803]::interface::Compiler>::enter::<rustc_driver_impl[8286e531aa15858]::run_compiler::{closure#1}::{closure#2}, core[8433f9a650d1cc7a]::result::Result<core[8433f9a650d1cc7a]::option::Option<rustc_interface[7a55d0aec9a4f803]::queries::Linker>, rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>
  32:     0x7f489ebba2b0 - rustc_span[8dcf7f7285d48a69]::set_source_map::<core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>, rustc_interface[7a55d0aec9a4f803]::interface::run_compiler<core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>, rustc_driver_impl[8286e531aa15858]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  33:     0x7f489ebac059 - <scoped_tls[9a8f34a7b1e3b707]::ScopedKey<rustc_span[8dcf7f7285d48a69]::SessionGlobals>>::set::<rustc_interface[7a55d0aec9a4f803]::interface::run_compiler<core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>, rustc_driver_impl[8286e531aa15858]::run_compiler::{closure#1}>::{closure#0}, core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>
  34:     0x7f489ebc4816 - std[267aca1effb3fcd0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a55d0aec9a4f803]::util::run_in_thread_pool_with_globals<rustc_interface[7a55d0aec9a4f803]::interface::run_compiler<core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>, rustc_driver_impl[8286e531aa15858]::run_compiler::{closure#1}>::{closure#0}, core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>
  35:     0x7f489ec01bf8 - std[267aca1effb3fcd0]::panicking::try::<core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>, core[8433f9a650d1cc7a]::panic::unwind_safe::AssertUnwindSafe<<std[267aca1effb3fcd0]::thread::Builder>::spawn_unchecked_<rustc_interface[7a55d0aec9a4f803]::util::run_in_thread_pool_with_globals<rustc_interface[7a55d0aec9a4f803]::interface::run_compiler<core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>, rustc_driver_impl[8286e531aa15858]::run_compiler::{closure#1}>::{closure#0}, core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  36:     0x7f489ebb21dd - <<std[267aca1effb3fcd0]::thread::Builder>::spawn_unchecked_<rustc_interface[7a55d0aec9a4f803]::util::run_in_thread_pool_with_globals<rustc_interface[7a55d0aec9a4f803]::interface::run_compiler<core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>, rustc_driver_impl[8286e531aa15858]::run_compiler::{closure#1}>::{closure#0}, core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8433f9a650d1cc7a]::result::Result<(), rustc_span[8dcf7f7285d48a69]::ErrorGuaranteed>>::{closure#1} as core[8433f9a650d1cc7a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f489e0eb3de - std::sys::unix::thread::Thread::new::thread_start::hfa0de9901275891c
  38:     0x7f489de83b43 - <unknown>
  39:     0x7f489df15a00 - <unknown>
  40:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (03d8dfaa8 2023-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C linker=x86_64-linux-gnu-gcc -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
