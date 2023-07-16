plain
failures:

---- [ui] src/test/ui-fulldeps/macro-crate-rlib.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/macro-crate-rlib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f88b3ef0b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66a4c7bad9a3cf14
   1:     0x7f88b3f59508 - core::fmt::write::h66e24808ab801029
   1:     0x7f88b3f59508 - core::fmt::write::h66e24808ab801029
   2:     0x7f88b3ee17d1 - std::io::Write::write_fmt::hbfdb296d6595ef0e
   3:     0x7f88b3ef3b5e - std::panicking::default_hook::{{closure}}::hed226cd6c5a9ea76
   4:     0x7f88b3ef3827 - std::panicking::default_hook::h189f2110f16ad97c
   5:     0x7f88b4878c44 - rustc_driver[e4456891b45b8452]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f88b3ef4311 - std::panicking::rust_panic_with_hook::h31c56e871a11b222
   7:     0x7f88b3ef4137 - std::panicking::begin_panic_handler::{{closure}}::h2509a444e856ec76
   8:     0x7f88b3ef10d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h34b9246f044f9102
   9:     0x7f88b3ef3e02 - rust_begin_unwind
  10:     0x7f88b3ea4e43 - core::panicking::panic_fmt::ha99589c8aa5dc8fb
  11:     0x7f88b3f55b71 - core::panicking::panic_display::h471a548b4b9d097a
  12:     0x7f88b3f55b1b - core::panicking::panic_str::h3a9640e4a1c3a1e0
  13:     0x7f88b3ea4cb6 - core::option::expect_failed::hf0a6d75aaf222a92
  14:     0x7f88b75767f2 - <rustc_errors[2e7bb0f12fb6f01a]::emitter::EmitterWriter as rustc_errors[2e7bb0f12fb6f01a]::translation::Translate>::translate_message
  15:     0x7f88b756b724 - <rustc_errors[2e7bb0f12fb6f01a]::emitter::EmitterWriter>::msg_to_buffer
  16:     0x7f88b756c0dc - <rustc_errors[2e7bb0f12fb6f01a]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f88b7569959 - <rustc_errors[2e7bb0f12fb6f01a]::emitter::EmitterWriter as rustc_errors[2e7bb0f12fb6f01a]::emitter::Emitter>::emit_diagnostic
  18:     0x7f88b7581d62 - <rustc_errors[2e7bb0f12fb6f01a]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f88b758077c - <rustc_errors[2e7bb0f12fb6f01a]::json::JsonEmitter as rustc_errors[2e7bb0f12fb6f01a]::emitter::Emitter>::emit_diagnostic
  20:     0x7f88b75c64c8 - <rustc_errors[2e7bb0f12fb6f01a]::HandlerInner>::emit_diagnostic
  21:     0x7f88b75c21b6 - <rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed as rustc_errors[2e7bb0f12fb6f01a]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f88b692db5b - <rustc_session[2c21920604463a77]::parse::ParseSess>::emit_err::<rustc_metadata[e0119640ef5b2756]::errors::NoDylibPlugin>
  23:     0x7f88b696d877 - <rustc_metadata[e0119640ef5b2756]::locator::CrateError>::report
  24:     0x7f88b696d3b2 - rustc_metadata[e0119640ef5b2756]::locator::find_plugin_registrar
  25:     0x7f88b68ecc31 - rustc_plugin_impl[75b3b6f0c9947dc4]::load::load_plugins
  26:     0x7f88b49fb2f1 - <rustc_session[2c21920604463a77]::session::Session>::time::<alloc[eb67d3b2bb253457]::vec::Vec<for<'a, 'b> fn(&'a mut rustc_plugin_impl[75b3b6f0c9947dc4]::Registry<'b>)>, rustc_interface[5c144df9c4d285bb]::passes::register_plugins<&dyn for<'a, 'b> core[bcf40f16c66fb259]::ops::function::Fn<(&'a rustc_session[2c21920604463a77]::session::Session, &'b mut rustc_lint[612935cd3add122e]::context::LintStore), Output = ()> + core[bcf40f16c66fb259]::marker::Sync + core[bcf40f16c66fb259]::marker::Send>::{closure#2}>
  27:     0x7f88b49b5cb1 - rustc_interface[5c144df9c4d285bb]::passes::register_plugins::<&dyn for<'a, 'b> core[bcf40f16c66fb259]::ops::function::Fn<(&'a rustc_session[2c21920604463a77]::session::Session, &'b mut rustc_lint[612935cd3add122e]::context::LintStore), Output = ()> + core[bcf40f16c66fb259]::marker::Sync + core[bcf40f16c66fb259]::marker::Send>
  28:     0x7f88b499c437 - <rustc_interface[5c144df9c4d285bb]::queries::Queries>::register_plugins
  29:     0x7f88b48818a9 - <rustc_interface[5c144df9c4d285bb]::interface::Compiler>::enter::<rustc_driver[e4456891b45b8452]::run_compiler::{closure#1}::{closure#2}, core[bcf40f16c66fb259]::result::Result<core[bcf40f16c66fb259]::option::Option<rustc_interface[5c144df9c4d285bb]::queries::Linker>, rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>>
  30:     0x7f88b48657f1 - rustc_span[4f584e5c0168705]::with_source_map::<core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>, rustc_interface[5c144df9c4d285bb]::interface::create_compiler_and_run<core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>, rustc_driver[e4456891b45b8452]::run_compiler::{closure#1}>::{closure#1}>
  31:     0x7f88b48836f1 - rustc_interface[5c144df9c4d285bb]::interface::create_compiler_and_run::<core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>, rustc_driver[e4456891b45b8452]::run_compiler::{closure#1}>
  32:     0x7f88b485d3c2 - <scoped_tls[100c020a455f98cd]::ScopedKey<rustc_span[4f584e5c0168705]::SessionGlobals>>::set::<rustc_interface[5c144df9c4d285bb]::interface::run_compiler<core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>, rustc_driver[e4456891b45b8452]::run_compiler::{closure#1}>::{closure#0}, core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>>
  33:     0x7f88b4869139 - std[32a96cd4c5ea00cf]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5c144df9c4d285bb]::util::run_in_thread_pool_with_globals<rustc_interface[5c144df9c4d285bb]::interface::run_compiler<core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>, rustc_driver[e4456891b45b8452]::run_compiler::{closure#1}>::{closure#0}, core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>>::{closure#0}, core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>>
  34:     0x7f88b48e396e - std[32a96cd4c5ea00cf]::panicking::try::<core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>, core[bcf40f16c66fb259]::panic::unwind_safe::AssertUnwindSafe<<std[32a96cd4c5ea00cf]::thread::Builder>::spawn_unchecked_<rustc_interface[5c144df9c4d285bb]::util::run_in_thread_pool_with_globals<rustc_interface[5c144df9c4d285bb]::interface::run_compiler<core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>, rustc_driver[e4456891b45b8452]::run_compiler::{closure#1}>::{closure#0}, core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>>::{closure#0}, core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f88b48def00 - <<std[32a96cd4c5ea00cf]::thread::Builder>::spawn_unchecked_<rustc_interface[5c144df9c4d285bb]::util::run_in_thread_pool_with_globals<rustc_interface[5c144df9c4d285bb]::interface::run_compiler<core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>, rustc_driver[e4456891b45b8452]::run_compiler::{closure#1}>::{closure#0}, core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>>::{closure#0}, core[bcf40f16c66fb259]::result::Result<(), rustc_errors[2e7bb0f12fb6f01a]::ErrorGuaranteed>>::{closure#1} as core[bcf40f16c66fb259]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f88b3f00b75 - std::sys::unix::thread::Thread::new::thread_start::hbb17793c636ae578
  37:     0x7f88b3c9eb43 - <unknown>
  38:     0x7f88b3d30a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (29e05950a 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

