plain
...........................i............................................................  8008/15027
....................................................i...................................  8096/15027
........................................................................................  8184/15027
........................................................................................  8272/15027
..............F.......................................................................ii  8360/15027
...............................i........................................................  8536/15027
........................................................................................  8624/15027
......................................................ii................................  8712/15027
........................................................................................  8800/15027
---
---- [ui] tests/ui/lto/issue-100772.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lto/issue-100772.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-100772/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-100772/auxiliary" "-Clto" "-Ctarget-feature=-crt-static" "-Zsanitizer=cfi"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs:685:13: encode_ty: unexpected `Param(T/#1)`
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1651:9
stack backtrace:
stack backtrace:
   0:     0x7fc741127aa4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2459a1c5a08b42a9
   1:     0x7fc74118e748 - core::fmt::write::hc2bb203c186edc01
   2:     0x7fc74111ca91 - std::io::Write::write_fmt::hdd4a4041852fc402
   3:     0x7fc7411278b1 - std::sys_common::backtrace::print::h1ea008923a930891
   4:     0x7fc74112a9da - std::panicking::default_hook::{{closure}}::hd9bb9e14e14770df
   5:     0x7fc74112a6ce - std::panicking::default_hook::hcc00a98b10132e2c
   6:     0x7fc741da512b - rustc_driver_impl[ff66ef544b6e7099]::install_ice_hook::{closure#0}
   7:     0x7fc74112b0b7 - std::panicking::rust_panic_with_hook::h0ce2a7b608599407
   8:     0x7fc74494f473 - std[ea5daadf13fee425]::panicking::begin_panic::<rustc_errors[51b79a47e13a5131]::ExplicitBug>::{closure#0}
   9:     0x7fc74494efd6 - std[ea5daadf13fee425]::sys_common::backtrace::__rust_end_short_backtrace::<std[ea5daadf13fee425]::panicking::begin_panic<rustc_errors[51b79a47e13a5131]::ExplicitBug>::{closure#0}, !>
  10:     0x7fc741d06de6 - std[ea5daadf13fee425]::panicking::begin_panic::<rustc_errors[51b79a47e13a5131]::ExplicitBug>
  11:     0x7fc7448c17a6 - std[ea5daadf13fee425]::panic::panic_any::<rustc_errors[51b79a47e13a5131]::ExplicitBug>
  12:     0x7fc7448bff2d - <rustc_errors[51b79a47e13a5131]::HandlerInner>::bug::<alloc[892ec1d74d922758]::string::String>
  13:     0x7fc7448bfd99 - <rustc_errors[51b79a47e13a5131]::Handler>::bug::<alloc[892ec1d74d922758]::string::String>
  14:     0x7fc7448b3fd0 - rustc_middle[f832314d2d2cec65]::util::bug::opt_span_bug_fmt::<rustc_span[a3f6a77aaae95b3a]::span_encoding::Span>::{closure#0}
  15:     0x7fc7448b2d4c - rustc_middle[f832314d2d2cec65]::ty::context::tls::with_opt::<rustc_middle[f832314d2d2cec65]::util::bug::opt_span_bug_fmt<rustc_span[a3f6a77aaae95b3a]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7fc7448b1f6e - rustc_middle[f832314d2d2cec65]::ty::context::tls::with_context_opt::<rustc_middle[f832314d2d2cec65]::ty::context::tls::with_opt<rustc_middle[f832314d2d2cec65]::util::bug::opt_span_bug_fmt<rustc_span[a3f6a77aaae95b3a]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7fc7448b2cf7 - rustc_middle[f832314d2d2cec65]::ty::context::tls::with_opt::<rustc_middle[f832314d2d2cec65]::util::bug::opt_span_bug_fmt<rustc_span[a3f6a77aaae95b3a]::span_encoding::Span>::{closure#0}, !>
  18:     0x7fc7448b3f09 - rustc_middle[f832314d2d2cec65]::util::bug::opt_span_bug_fmt::<rustc_span[a3f6a77aaae95b3a]::span_encoding::Span>
  19:     0x7fc741cf4125 - rustc_middle[f832314d2d2cec65]::util::bug::bug_fmt
  20:     0x7fc743c85b99 - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_itanium_cxx_abi::encode_ty
  21:     0x7fc743c84dd7 - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_itanium_cxx_abi::encode_ty
  22:     0x7fc743c836f6 - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_itanium_cxx_abi::encode_substs
  23:     0x7fc743c82f21 - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_itanium_cxx_abi::encode_predicates
  24:     0x7fc743c84b52 - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_itanium_cxx_abi::encode_ty
  25:     0x7fc743c84dd7 - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_itanium_cxx_abi::encode_ty
  26:     0x7fc743c8813d - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_itanium_cxx_abi::typeid_for_fnabi
  27:     0x7fc743c872b8 - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_itanium_cxx_abi::typeid_for_instance
  28:     0x7fc743c54539 - rustc_symbol_mangling[37e592e4b09e07e4]::typeid::typeid_for_instance
  29:     0x7fc7420335d4 - <rustc_codegen_llvm[17215c05f881e532]::context::CodegenCx>::declare_fn
  30:     0x7fc7420344f1 - <rustc_codegen_llvm[17215c05f881e532]::context::CodegenCx as rustc_codegen_ssa[6cdcf7839c259b0c]::traits::declare::PreDefineMethods>::predefine_fn
  31:     0x7fc7420d0ee4 - <rustc_middle[f832314d2d2cec65]::mir::mono::MonoItem as rustc_codegen_ssa[6cdcf7839c259b0c]::mono_item::MonoItemExt>::predefine::<rustc_codegen_llvm[17215c05f881e532]::builder::Builder>
  32:     0x7fc741fa77b2 - rustc_codegen_llvm[17215c05f881e532]::base::compile_codegen_unit::module_codegen
  33:     0x7fc741fa6b2d - rustc_codegen_llvm[17215c05f881e532]::base::compile_codegen_unit
  34:     0x7fc741f6399a - rustc_codegen_ssa[6cdcf7839c259b0c]::base::codegen_crate::<rustc_codegen_llvm[17215c05f881e532]::LlvmCodegenBackend>
  35:     0x7fc7420db8d4 - <rustc_codegen_llvm[17215c05f881e532]::LlvmCodegenBackend as rustc_codegen_ssa[6cdcf7839c259b0c]::traits::backend::CodegenBackend>::codegen_crate
  36:     0x7fc741eb573f - rustc_interface[abc282a9b0dab5cd]::passes::start_codegen
  37:     0x7fc741e9293f - <std[ea5daadf13fee425]::thread::local::LocalKey<core[a29f621235a26afc]::cell::Cell<*const ()>>>::with::<rustc_middle[f832314d2d2cec65]::ty::context::tls::enter_context<<rustc_middle[f832314d2d2cec65]::ty::context::GlobalCtxt>::enter<<rustc_interface[abc282a9b0dab5cd]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[a29f621235a26afc]::result::Result<alloc[892ec1d74d922758]::boxed::Box<dyn core[a29f621235a26afc]::any::Any>, rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>::{closure#0}, core[a29f621235a26afc]::result::Result<alloc[892ec1d74d922758]::boxed::Box<dyn core[a29f621235a26afc]::any::Any>, rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>::{closure#0}, core[a29f621235a26afc]::result::Result<alloc[892ec1d74d922758]::boxed::Box<dyn core[a29f621235a26afc]::any::Any>, rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>
  38:     0x7fc741e799fc - <rustc_interface[abc282a9b0dab5cd]::queries::Queries>::ongoing_codegen
  39:     0x7fc741e0e12a - <rustc_interface[abc282a9b0dab5cd]::interface::Compiler>::enter::<rustc_driver_impl[ff66ef544b6e7099]::run_compiler::{closure#1}::{closure#2}, core[a29f621235a26afc]::result::Result<core[a29f621235a26afc]::option::Option<rustc_interface[abc282a9b0dab5cd]::queries::Linker>, rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>
  40:     0x7fc741df5ab2 - <scoped_tls[effe0779932644a0]::ScopedKey<rustc_span[a3f6a77aaae95b3a]::SessionGlobals>>::set::<rustc_interface[abc282a9b0dab5cd]::interface::run_compiler<core[a29f621235a26afc]::result::Result<(), rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>, rustc_driver_impl[ff66ef544b6e7099]::run_compiler::{closure#1}>::{closure#0}, core[a29f621235a26afc]::result::Result<(), rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>
  41:     0x7fc741dc5079 - std[ea5daadf13fee425]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[abc282a9b0dab5cd]::util::run_in_thread_pool_with_globals<rustc_interface[abc282a9b0dab5cd]::interface::run_compiler<core[a29f621235a26afc]::result::Result<(), rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>, rustc_driver_impl[ff66ef544b6e7099]::run_compiler::{closure#1}>::{closure#0}, core[a29f621235a26afc]::result::Result<(), rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a29f621235a26afc]::result::Result<(), rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>
  42:     0x7fc741dc6f69 - <<std[ea5daadf13fee425]::thread::Builder>::spawn_unchecked_<rustc_interface[abc282a9b0dab5cd]::util::run_in_thread_pool_with_globals<rustc_interface[abc282a9b0dab5cd]::interface::run_compiler<core[a29f621235a26afc]::result::Result<(), rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>, rustc_driver_impl[ff66ef544b6e7099]::run_compiler::{closure#1}>::{closure#0}, core[a29f621235a26afc]::result::Result<(), rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a29f621235a26afc]::result::Result<(), rustc_span[a3f6a77aaae95b3a]::ErrorGuaranteed>>::{closure#1} as core[a29f621235a26afc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7fc741136f8e - std::sys::unix::thread::Thread::new::thread_start::h99fb7b430520f378
  44:     0x7fc740ed3b43 - <unknown>
  45:     0x7fc740f65a00 - <unknown>
  46:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (cddc01330 2023-05-20) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C rpath -C debuginfo=0 -C lto -C target-feature=-crt-static -Z sanitizer=cfi
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
