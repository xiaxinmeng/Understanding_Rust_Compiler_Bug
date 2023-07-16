plain
---- [ui] tests/ui/lto/issue-100772.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lto/issue-100772.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-100772/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-100772/auxiliary" "-Clto" "-Ctarget-feature=-crt-static" "-Zsanitizer=cfi"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:1055:17: typeid_for_instance: concrete self isn't a reference ()
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7faab18283f4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5a7af9977f908956
   1:     0x7faab188efe8 - core::fmt::write::hb833f2f5d3e2aeac
   1:     0x7faab188efe8 - core::fmt::write::hb833f2f5d3e2aeac
   2:     0x7faab181cba1 - std::io::Write::write_fmt::hd1377392f075c78e
   3:     0x7faab1828201 - std::sys_common::backtrace::print::h7ca1b981b2dc44d6
   4:     0x7faab182b38a - std::panicking::default_hook::{{closure}}::h26299c9413c39bc3
   5:     0x7faab182b06c - std::panicking::default_hook::h416bf98e088b628f
   6:     0x7faab22f1fbb - rustc_driver_impl[668ff06edc8fd8b9]::install_ice_hook::{closure#0}
   7:     0x7faab182ba97 - std::panicking::rust_panic_with_hook::h65bdc17ae80f5727
   8:     0x7faab4f6d443 - std[98751b8be19751c1]::panicking::begin_panic::<rustc_errors[a15977436d1ad8ad]::ExplicitBug>::{closure#0}
   9:     0x7faab4f69456 - std[98751b8be19751c1]::sys_common::backtrace::__rust_end_short_backtrace::<std[98751b8be19751c1]::panicking::begin_panic<rustc_errors[a15977436d1ad8ad]::ExplicitBug>::{closure#0}, !>
  10:     0x7faab2289ec6 - std[98751b8be19751c1]::panicking::begin_panic::<rustc_errors[a15977436d1ad8ad]::ExplicitBug>
  11:     0x7faab4f5d4e7 - <rustc_errors[a15977436d1ad8ad]::HandlerInner>::bug::<alloc[5a66e5a8e0224cd4]::string::String>
  12:     0x7faab4f5c579 - <rustc_errors[a15977436d1ad8ad]::Handler>::bug::<alloc[5a66e5a8e0224cd4]::string::String>
  13:     0x7faab510c987 - rustc_middle[7ac8d09f958d6997]::util::bug::opt_span_bug_fmt::<rustc_span[412730f143c8490d]::span_encoding::Span>::{closure#0}
  14:     0x7faab510ba5c - rustc_middle[7ac8d09f958d6997]::ty::context::tls::with_opt::<rustc_middle[7ac8d09f958d6997]::util::bug::opt_span_bug_fmt<rustc_span[412730f143c8490d]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7faab510b9e4 - rustc_middle[7ac8d09f958d6997]::ty::context::tls::with_context_opt::<rustc_middle[7ac8d09f958d6997]::ty::context::tls::with_opt<rustc_middle[7ac8d09f958d6997]::util::bug::opt_span_bug_fmt<rustc_span[412730f143c8490d]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7faab2294422 - rustc_middle[7ac8d09f958d6997]::util::bug::bug_fmt
  17:     0x7faab42b6aa5 - rustc_symbol_mangling[3108b4ff2a0da511]::typeid::typeid_itanium_cxx_abi::typeid_for_instance
  18:     0x7faab4283ec9 - rustc_symbol_mangling[3108b4ff2a0da511]::typeid::typeid_for_instance
  19:     0x7faab2592d50 - <rustc_codegen_llvm[43dec02245a8480e]::context::CodegenCx>::declare_fn
  20:     0x7faab2593844 - <rustc_codegen_llvm[43dec02245a8480e]::context::CodegenCx as rustc_codegen_ssa[bd0a33fba0e22275]::traits::declare::PreDefineMethods>::predefine_fn
  21:     0x7faab25c696a - <rustc_middle[7ac8d09f958d6997]::mir::mono::MonoItem as rustc_codegen_ssa[bd0a33fba0e22275]::mono_item::MonoItemExt>::predefine::<rustc_codegen_llvm[43dec02245a8480e]::builder::Builder>
  22:     0x7faab24c58c2 - rustc_codegen_llvm[43dec02245a8480e]::base::compile_codegen_unit::module_codegen
  23:     0x7faab24c47a0 - rustc_codegen_llvm[43dec02245a8480e]::base::compile_codegen_unit
  24:     0x7faab259b78a - rustc_codegen_ssa[bd0a33fba0e22275]::base::codegen_crate::<rustc_codegen_llvm[43dec02245a8480e]::LlvmCodegenBackend>
  25:     0x7faab256b7c0 - <rustc_codegen_llvm[43dec02245a8480e]::LlvmCodegenBackend as rustc_codegen_ssa[bd0a33fba0e22275]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7faab23e197f - rustc_interface[cf7a041d1490a8d9]::passes::start_codegen
  27:     0x7faab23eeb96 - <rustc_middle[7ac8d09f958d6997]::ty::context::GlobalCtxt>::enter::<<rustc_interface[cf7a041d1490a8d9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[2fc6cb230af0153b]::result::Result<alloc[5a66e5a8e0224cd4]::boxed::Box<dyn core[2fc6cb230af0153b]::any::Any>, rustc_span[412730f143c8490d]::ErrorGuaranteed>>
  28:     0x7faab23bfab4 - <rustc_interface[cf7a041d1490a8d9]::queries::Queries>::ongoing_codegen
  29:     0x7faab2343a26 - <rustc_interface[cf7a041d1490a8d9]::interface::Compiler>::enter::<rustc_driver_impl[668ff06edc8fd8b9]::run_compiler::{closure#1}::{closure#2}, core[2fc6cb230af0153b]::result::Result<core[2fc6cb230af0153b]::option::Option<rustc_interface[cf7a041d1490a8d9]::queries::Linker>, rustc_span[412730f143c8490d]::ErrorGuaranteed>>
  30:     0x7faab230cd90 - rustc_span[412730f143c8490d]::set_source_map::<core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>, rustc_interface[cf7a041d1490a8d9]::interface::run_compiler<core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>, rustc_driver_impl[668ff06edc8fd8b9]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  31:     0x7faab22ff649 - <scoped_tls[b22129679dff0778]::ScopedKey<rustc_span[412730f143c8490d]::SessionGlobals>>::set::<rustc_interface[cf7a041d1490a8d9]::interface::run_compiler<core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>, rustc_driver_impl[668ff06edc8fd8b9]::run_compiler::{closure#1}>::{closure#0}, core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>>
  32:     0x7faab2316766 - std[98751b8be19751c1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf7a041d1490a8d9]::util::run_in_thread_pool_with_globals<rustc_interface[cf7a041d1490a8d9]::interface::run_compiler<core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>, rustc_driver_impl[668ff06edc8fd8b9]::run_compiler::{closure#1}>::{closure#0}, core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>>
  33:     0x7faab2355526 - std[98751b8be19751c1]::panicking::try::<core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>, core[2fc6cb230af0153b]::panic::unwind_safe::AssertUnwindSafe<<std[98751b8be19751c1]::thread::Builder>::spawn_unchecked_<rustc_interface[cf7a041d1490a8d9]::util::run_in_thread_pool_with_globals<rustc_interface[cf7a041d1490a8d9]::interface::run_compiler<core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>, rustc_driver_impl[668ff06edc8fd8b9]::run_compiler::{closure#1}>::{closure#0}, core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7faab23057ad - <<std[98751b8be19751c1]::thread::Builder>::spawn_unchecked_<rustc_interface[cf7a041d1490a8d9]::util::run_in_thread_pool_with_globals<rustc_interface[cf7a041d1490a8d9]::interface::run_compiler<core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>, rustc_driver_impl[668ff06edc8fd8b9]::run_compiler::{closure#1}>::{closure#0}, core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2fc6cb230af0153b]::result::Result<(), rustc_span[412730f143c8490d]::ErrorGuaranteed>>::{closure#1} as core[2fc6cb230af0153b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7faab18383ce - std::sys::unix::thread::Thread::new::thread_start::hdf11cfaad716287b
  36:     0x7faab15d3b43 - <unknown>
  37:     0x7faab1665a00 - <unknown>
  38:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (f08ace279 2023-05-08) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C rpath -C debuginfo=0 -C lto -C target-feature=-crt-static -Z sanitizer=cfi
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
