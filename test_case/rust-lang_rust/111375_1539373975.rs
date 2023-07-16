plain
---- [ui] tests/ui/lto/issue-100772.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lto/issue-100772.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-100772/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-100772/auxiliary" "-Clto" "-Ctarget-feature=-crt-static" "-Zsanitizer=cfi"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:1029:33
   0:     0x7f7c69f033f4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a856f2d8a40858d
   1:     0x7f7c69f6a008 - core::fmt::write::he81055b6e404098c
   2:     0x7f7c69ef7ba1 - std::io::Write::write_fmt::he545ac970b36d98c
   3:     0x7f7c69f03201 - std::sys_common::backtrace::print::ha0712e9b0889bd83
   3:     0x7f7c69f03201 - std::sys_common::backtrace::print::ha0712e9b0889bd83
   4:     0x7f7c69f0638a - std::panicking::default_hook::{{closure}}::hbe81dff82f6309e3
   5:     0x7f7c69f0606c - std::panicking::default_hook::hd619c49c00b35415
   6:     0x7f7c6a9d063b - rustc_driver_impl[1f7179344093384d]::install_ice_hook::{closure#0}
   7:     0x7f7c69f06a97 - std::panicking::rust_panic_with_hook::haa322785608513c9
   8:     0x7f7c69f06817 - std::panicking::begin_panic_handler::{{closure}}::ha48f51bc22b6b030
   9:     0x7f7c69f038d6 - std::sys_common::backtrace::__rust_end_short_backtrace::h2b86d1563670b20c
  10:     0x7f7c69f06507 - rust_begin_unwind
  11:     0x7f7c69ebb0c3 - core::panicking::panic_fmt::h3663b6a0d58a64c2
  12:     0x7f7c69ebb222 - core::panicking::panic_bounds_check::h26c3d2fd84213aff
  13:     0x7f7c6c99ef99 - rustc_symbol_mangling[f3d187d1d7f159c4]::typeid::typeid_itanium_cxx_abi::typeid_for_instance
  14:     0x7f7c6c96c2c9 - rustc_symbol_mangling[f3d187d1d7f159c4]::typeid::typeid_for_instance
  15:     0x7f7c6ac721b0 - <rustc_codegen_llvm[3daf47b383783e72]::context::CodegenCx>::declare_fn
  16:     0x7f7c6ac72ca4 - <rustc_codegen_llvm[3daf47b383783e72]::context::CodegenCx as rustc_codegen_ssa[132fc3f9b6b6fdf2]::traits::declare::PreDefineMethods>::predefine_fn
  17:     0x7f7c6aca5d9a - <rustc_middle[a1d3934e196e3db5]::mir::mono::MonoItem as rustc_codegen_ssa[132fc3f9b6b6fdf2]::mono_item::MonoItemExt>::predefine::<rustc_codegen_llvm[3daf47b383783e72]::builder::Builder>
  18:     0x7f7c6aba4c92 - rustc_codegen_llvm[3daf47b383783e72]::base::compile_codegen_unit::module_codegen
  19:     0x7f7c6aba3b80 - rustc_codegen_llvm[3daf47b383783e72]::base::compile_codegen_unit
  20:     0x7f7c6ac7828a - rustc_codegen_ssa[132fc3f9b6b6fdf2]::base::codegen_crate::<rustc_codegen_llvm[3daf47b383783e72]::LlvmCodegenBackend>
  21:     0x7f7c6ac4ac10 - <rustc_codegen_llvm[3daf47b383783e72]::LlvmCodegenBackend as rustc_codegen_ssa[132fc3f9b6b6fdf2]::traits::backend::CodegenBackend>::codegen_crate
  22:     0x7f7c6aac0f8f - rustc_interface[bee8fce5eb70df5d]::passes::start_codegen
  23:     0x7f7c6aace1f6 - <rustc_middle[a1d3934e196e3db5]::ty::context::GlobalCtxt>::enter::<<rustc_interface[bee8fce5eb70df5d]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[4109acd62fa289cb]::result::Result<alloc[d9ffa471352628f3]::boxed::Box<dyn core[4109acd62fa289cb]::any::Any>, rustc_span[6795e9645d180f32]::ErrorGuaranteed>>
  24:     0x7f7c6aa9f0a4 - <rustc_interface[bee8fce5eb70df5d]::queries::Queries>::ongoing_codegen
  25:     0x7f7c6aa23156 - <rustc_interface[bee8fce5eb70df5d]::interface::Compiler>::enter::<rustc_driver_impl[1f7179344093384d]::run_compiler::{closure#1}::{closure#2}, core[4109acd62fa289cb]::result::Result<core[4109acd62fa289cb]::option::Option<rustc_interface[bee8fce5eb70df5d]::queries::Linker>, rustc_span[6795e9645d180f32]::ErrorGuaranteed>>
  26:     0x7f7c6a9eb410 - rustc_span[6795e9645d180f32]::set_source_map::<core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>, rustc_interface[bee8fce5eb70df5d]::interface::run_compiler<core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>, rustc_driver_impl[1f7179344093384d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7f7c6a9ddcc9 - <scoped_tls[3c55af6dbe48b1f1]::ScopedKey<rustc_span[6795e9645d180f32]::SessionGlobals>>::set::<rustc_interface[bee8fce5eb70df5d]::interface::run_compiler<core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>, rustc_driver_impl[1f7179344093384d]::run_compiler::{closure#1}>::{closure#0}, core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>>
  28:     0x7f7c6aa1d2e6 - std[199939ab6deaac05]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bee8fce5eb70df5d]::util::run_in_thread_pool_with_globals<rustc_interface[bee8fce5eb70df5d]::interface::run_compiler<core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>, rustc_driver_impl[1f7179344093384d]::run_compiler::{closure#1}>::{closure#0}, core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>>
  29:     0x7f7c6aa23906 - std[199939ab6deaac05]::panicking::try::<core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>, core[4109acd62fa289cb]::panic::unwind_safe::AssertUnwindSafe<<std[199939ab6deaac05]::thread::Builder>::spawn_unchecked_<rustc_interface[bee8fce5eb70df5d]::util::run_in_thread_pool_with_globals<rustc_interface[bee8fce5eb70df5d]::interface::run_compiler<core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>, rustc_driver_impl[1f7179344093384d]::run_compiler::{closure#1}>::{closure#0}, core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  30:     0x7f7c6a9e3e2d - <<std[199939ab6deaac05]::thread::Builder>::spawn_unchecked_<rustc_interface[bee8fce5eb70df5d]::util::run_in_thread_pool_with_globals<rustc_interface[bee8fce5eb70df5d]::interface::run_compiler<core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>, rustc_driver_impl[1f7179344093384d]::run_compiler::{closure#1}>::{closure#0}, core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4109acd62fa289cb]::result::Result<(), rustc_span[6795e9645d180f32]::ErrorGuaranteed>>::{closure#1} as core[4109acd62fa289cb]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f7c69f133ce - std::sys::unix::thread::Thread::new::thread_start::ha8c5c2fb1f3bf593
  32:     0x7f7c69cadb43 - <unknown>
  33:     0x7f7c69d3fa00 - <unknown>
  34:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (e1f3108ba 2023-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C rpath -C debuginfo=0 -C lto -C target-feature=-crt-static -Z sanitizer=cfi
query stack during panic:
end of query stack
------------------------------------------

