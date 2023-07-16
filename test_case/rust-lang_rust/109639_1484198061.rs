plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:13c1b4e09b845ddb9664cee13d03879444a1054d)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.87
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
thread 'rustc' panicked at 'no CodegenUnitDebugContext found on LLVM context', compiler/rustc_codegen_llvm/src/debuginfo/mod.rs:298:39
stack backtrace:
   0:     0x7f0b4d041425 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h58f3d5f9613fc903
   1:     0x7f0b4d0add58 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f0b4d035731 - std::io::Write::write_fmt::h46e9ff7509b654e1
   3:     0x7f0b4d041235 - std::sys_common::backtrace::print::h0b48412da775984a
   4:     0x7f0b4d0443f4 - std::panicking::default_hook::{{closure}}::h4eaef8437e4e66b7
   5:     0x7f0b4d0440e2 - std::panicking::default_hook::hef252b4dd17d3bdf
   6:     0x7f0b4daca2b5 - rustc_driver_impl[5ab6970a15cdd324]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f0b4d044b29 - std::panicking::rust_panic_with_hook::hff848c0b52cb878e
   8:     0x7f0b4d0448a9 - std::panicking::begin_panic_handler::{{closure}}::h35a687b7fe1e45d3
   9:     0x7f0b4d0418d6 - std::sys_common::backtrace::__rust_end_short_backtrace::h582581d5cc1315f5
  10:     0x7f0b4d044572 - rust_begin_unwind
  11:     0x7f0b4cffe0b3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f0b4d0a9c81 - core::panicking::panic_display::hd6137c9d9b8fef57
  13:     0x7f0b4d0a9c2b - core::panicking::panic_str::hb08c981e56dec499
  14:     0x7f0b4cffe076 - core::option::expect_failed::h24ffd4a15f50a4ac
  15:     0x7f0b4dda81f9 - <rustc_codegen_llvm[ad916b057f2f8375]::context::CodegenCx as rustc_codegen_ssa[d284098dc66eb442]::traits::statics::StaticMethods>::codegen_static
  16:     0x7f0b4ddfd4a2 - <rustc_middle[ef4d301d6aa1f8df]::mir::mono::MonoItem as rustc_codegen_ssa[d284098dc66eb442]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[ad916b057f2f8375]::builder::Builder>
  17:     0x7f0b4dce63b0 - rustc_codegen_llvm[ad916b057f2f8375]::base::compile_codegen_unit::module_codegen
  18:     0x7f0b4dce57a0 - rustc_codegen_llvm[ad916b057f2f8375]::base::compile_codegen_unit
  19:     0x7f0b4ddba755 - rustc_codegen_ssa[d284098dc66eb442]::base::codegen_crate::<rustc_codegen_llvm[ad916b057f2f8375]::LlvmCodegenBackend>
  20:     0x7f0b4dd873bd - <rustc_codegen_llvm[ad916b057f2f8375]::LlvmCodegenBackend as rustc_codegen_ssa[d284098dc66eb442]::traits::backend::CodegenBackend>::codegen_crate
  21:     0x7f0b4dc0875f - <rustc_session[9db97d1686892f5f]::session::Session>::time::<alloc[5a8e793bc0d22b6e]::boxed::Box<dyn core[54ab13d2a06817e1]::any::Any>, rustc_interface[ea4bed2aa28b4af0]::passes::start_codegen::{closure#0}>
  22:     0x7f0b4dbbf271 - rustc_interface[ea4bed2aa28b4af0]::passes::start_codegen
  23:     0x7f0b4dbc412a - <rustc_middle[ef4d301d6aa1f8df]::ty::context::GlobalCtxt>::enter::<<rustc_interface[ea4bed2aa28b4af0]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<alloc[5a8e793bc0d22b6e]::boxed::Box<dyn core[54ab13d2a06817e1]::any::Any>, rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  24:     0x7f0b4dca5a04 - <rustc_interface[ea4bed2aa28b4af0]::queries::Queries>::ongoing_codegen
  25:     0x7f0b4db1681f - <rustc_interface[ea4bed2aa28b4af0]::interface::Compiler>::enter::<rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[ea4bed2aa28b4af0]::queries::Linker>, rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  26:     0x7f0b4db5ac60 - rustc_span[7f1d9ddd6ed7818e]::with_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7f0b4dae71d9 - std[e292dd3bffb96032]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ea4bed2aa28b4af0]::util::run_in_thread_pool_with_globals<rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>
  28:     0x7f0b4dae8053 - <<std[e292dd3bffb96032]::thread::Builder>::spawn_unchecked_<rustc_interface[ea4bed2aa28b4af0]::util::run_in_thread_pool_with_globals<rustc_interface[ea4bed2aa28b4af0]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>, rustc_driver_impl[5ab6970a15cdd324]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7f1d9ddd6ed7818e]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  29:     0x7f0b4d050e8e - std::sys::unix::thread::Thread::new::thread_start::hb98a15cf8f2a3509
  30:     0x7f0b4cdeeb43 - <unknown>
  31:     0x7f0b4ce80a00 - <unknown>
  32:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (f024a7989 2023-03-26) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
