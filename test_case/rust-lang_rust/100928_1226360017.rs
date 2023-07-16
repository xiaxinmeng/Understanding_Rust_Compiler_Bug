plain
failures:

---- [ui] src/test/ui-fulldeps/macro-crate-rlib.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/macro-crate-rlib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "metadata_incompatible_rustc", attr: None, args: FluentArgs([("crate_name", String("rlib_crate_test"))]), errors: [ResolverError(Reference(Variable { id: "add_info" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f07d09c6b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd63ae0b5cdefbf61
   1:     0x7f07d0a2f508 - core::fmt::write::hd644d65d76bd07c5
   2:     0x7f07d09b7731 - std::io::Write::write_fmt::h5e8ec6989b4b4b7d
   2:     0x7f07d09b7731 - std::io::Write::write_fmt::h5e8ec6989b4b4b7d
   3:     0x7f07d09c9b5e - std::panicking::default_hook::{{closure}}::hd392fec6cf0966d2
   4:     0x7f07d09c9827 - std::panicking::default_hook::h8e5910e6a22655f0
   5:     0x7f07d134eb94 - rustc_driver[e8deecba18e1726d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f07d09ca311 - std::panicking::rust_panic_with_hook::h3511465374f46c10
   7:     0x7f07d09ca137 - std::panicking::begin_panic_handler::{{closure}}::h257755d41ccec935
   8:     0x7f07d09c70d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h2a75232ed612de2f
   9:     0x7f07d09c9e02 - rust_begin_unwind
  10:     0x7f07d097ae43 - core::panicking::panic_fmt::hafdd9f2f8f014818
  11:     0x7f07d4047627 - <rustc_errors[1714a297db23f65d]::emitter::EmitterWriter as rustc_errors[1714a297db23f65d]::translation::Translate>::translate_message
  12:     0x7f07d403ca72 - <rustc_errors[1714a297db23f65d]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f07d403a516 - <rustc_errors[1714a297db23f65d]::emitter::EmitterWriter as rustc_errors[1714a297db23f65d]::emitter::Emitter>::emit_diagnostic
  14:     0x7f07d40528e2 - <rustc_errors[1714a297db23f65d]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f07d40512fc - <rustc_errors[1714a297db23f65d]::json::JsonEmitter as rustc_errors[1714a297db23f65d]::emitter::Emitter>::emit_diagnostic
  16:     0x7f07d4097008 - <rustc_errors[1714a297db23f65d]::HandlerInner>::emit_diagnostic
  17:     0x7f07d4092cf6 - <rustc_errors[1714a297db23f65d]::ErrorGuaranteed as rustc_errors[1714a297db23f65d]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f07d33fe664 - <rustc_session[ed1d893e6c68e3da]::parse::ParseSess>::emit_err::<rustc_metadata[5f9c7ceacb807d1c]::errors::NonDylibPlugin>
  19:     0x7f07d343d7a7 - <rustc_metadata[5f9c7ceacb807d1c]::locator::CrateError>::report
  20:     0x7f07d343d2e2 - rustc_metadata[5f9c7ceacb807d1c]::locator::find_plugin_registrar
  21:     0x7f07d33bcb81 - rustc_plugin_impl[cd5fe5d8e397a374]::load::load_plugins
  22:     0x7f07d14d0861 - <rustc_session[ed1d893e6c68e3da]::session::Session>::time::<alloc[365189f74199fd97]::vec::Vec<for<'a, 'b> fn(&'a mut rustc_plugin_impl[cd5fe5d8e397a374]::Registry<'b>)>, rustc_interface[2fe75ff7a808cf58]::passes::register_plugins<&dyn for<'a, 'b> core[c062d1a1249a8283]::ops::function::Fn<(&'a rustc_session[ed1d893e6c68e3da]::session::Session, &'b mut rustc_lint[52b776718897dfa5]::context::LintStore), Output = ()> + core[c062d1a1249a8283]::marker::Sync + core[c062d1a1249a8283]::marker::Send>::{closure#2}>
  23:     0x7f07d148bc21 - rustc_interface[2fe75ff7a808cf58]::passes::register_plugins::<&dyn for<'a, 'b> core[c062d1a1249a8283]::ops::function::Fn<(&'a rustc_session[ed1d893e6c68e3da]::session::Session, &'b mut rustc_lint[52b776718897dfa5]::context::LintStore), Output = ()> + core[c062d1a1249a8283]::marker::Sync + core[c062d1a1249a8283]::marker::Send>
  24:     0x7f07d1472407 - <rustc_interface[2fe75ff7a808cf58]::queries::Queries>::register_plugins
  25:     0x7f07d1357879 - <rustc_interface[2fe75ff7a808cf58]::interface::Compiler>::enter::<rustc_driver[e8deecba18e1726d]::run_compiler::{closure#1}::{closure#2}, core[c062d1a1249a8283]::result::Result<core[c062d1a1249a8283]::option::Option<rustc_interface[2fe75ff7a808cf58]::queries::Linker>, rustc_errors[1714a297db23f65d]::ErrorGuaranteed>>
  26:     0x7f07d133b7c1 - rustc_span[80fe21c7ba5b6fac]::with_source_map::<core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>, rustc_interface[2fe75ff7a808cf58]::interface::create_compiler_and_run<core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>, rustc_driver[e8deecba18e1726d]::run_compiler::{closure#1}>::{closure#1}>
  27:     0x7f07d13596b1 - rustc_interface[2fe75ff7a808cf58]::interface::create_compiler_and_run::<core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>, rustc_driver[e8deecba18e1726d]::run_compiler::{closure#1}>
  28:     0x7f07d1333392 - <scoped_tls[1ebf85ad32b966a1]::ScopedKey<rustc_span[80fe21c7ba5b6fac]::SessionGlobals>>::set::<rustc_interface[2fe75ff7a808cf58]::interface::run_compiler<core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>, rustc_driver[e8deecba18e1726d]::run_compiler::{closure#1}>::{closure#0}, core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>>
  29:     0x7f07d133f039 - std[cbd9db4efa0e8689]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2fe75ff7a808cf58]::util::run_in_thread_pool_with_globals<rustc_interface[2fe75ff7a808cf58]::interface::run_compiler<core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>, rustc_driver[e8deecba18e1726d]::run_compiler::{closure#1}>::{closure#0}, core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>>::{closure#0}, core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>>
  30:     0x7f07d13b9d6e - std[cbd9db4efa0e8689]::panicking::try::<core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>, core[c062d1a1249a8283]::panic::unwind_safe::AssertUnwindSafe<<std[cbd9db4efa0e8689]::thread::Builder>::spawn_unchecked_<rustc_interface[2fe75ff7a808cf58]::util::run_in_thread_pool_with_globals<rustc_interface[2fe75ff7a808cf58]::interface::run_compiler<core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>, rustc_driver[e8deecba18e1726d]::run_compiler::{closure#1}>::{closure#0}, core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>>::{closure#0}, core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  31:     0x7f07d13b4ed0 - <<std[cbd9db4efa0e8689]::thread::Builder>::spawn_unchecked_<rustc_interface[2fe75ff7a808cf58]::util::run_in_thread_pool_with_globals<rustc_interface[2fe75ff7a808cf58]::interface::run_compiler<core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>, rustc_driver[e8deecba18e1726d]::run_compiler::{closure#1}>::{closure#0}, core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>>::{closure#0}, core[c062d1a1249a8283]::result::Result<(), rustc_errors[1714a297db23f65d]::ErrorGuaranteed>>::{closure#1} as core[c062d1a1249a8283]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f07d09d6b75 - std::sys::unix::thread::Thread::new::thread_start::h3da403b82495afd4
  33:     0x7f07d0774b43 - <unknown>
  34:     0x7f07d0806a00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (1fcd6fdda 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

