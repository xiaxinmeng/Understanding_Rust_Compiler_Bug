plain
failures:

---- [ui] src/test/ui-fulldeps/macro-crate-rlib.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/macro-crate-rlib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7f672d19bb5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6cfe1c01d9cd1dad
   1:     0x7f672d204508 - core::fmt::write::haddea0eef2c60588
   2:     0x7f672d18c731 - std::io::Write::write_fmt::h605c06df43bc8b1a
   3:     0x7f672d19eb5e - std::panicking::default_hook::{{closure}}::h3b75bf462d6dfdf2
   4:     0x7f672d19e827 - std::panicking::default_hook::h1a1ffb442b2c23fd
   5:     0x7f672db23c34 - rustc_driver[b81b286ec3c48051]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f672d19f311 - std::panicking::rust_panic_with_hook::h94c1a470a1fcaa73
   7:     0x7f672d19f137 - std::panicking::begin_panic_handler::{{closure}}::hed5d1700f66f4b87
   8:     0x7f672d19c0d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h669ccfb8fce14899
   9:     0x7f672d19ee02 - rust_begin_unwind
  10:     0x7f672d14fe43 - core::panicking::panic_fmt::h4bc8d32e8133c219
  11:     0x7f672d200b71 - core::panicking::panic_display::hafb4c48825ca517d
  12:     0x7f672d200b1b - core::panicking::panic_str::h3e5d52b931d721fd
  13:     0x7f672d14fcb6 - core::option::expect_failed::hb502016698d2c33a
  14:     0x7f6730820112 - <rustc_errors[9cfc93648b64d3e2]::emitter::EmitterWriter as rustc_errors[9cfc93648b64d3e2]::translation::Translate>::translate_message
  15:     0x7f6730815044 - <rustc_errors[9cfc93648b64d3e2]::emitter::EmitterWriter>::msg_to_buffer
  16:     0x7f67308159fc - <rustc_errors[9cfc93648b64d3e2]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f6730813279 - <rustc_errors[9cfc93648b64d3e2]::emitter::EmitterWriter as rustc_errors[9cfc93648b64d3e2]::emitter::Emitter>::emit_diagnostic
  18:     0x7f673082b682 - <rustc_errors[9cfc93648b64d3e2]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f673082a09c - <rustc_errors[9cfc93648b64d3e2]::json::JsonEmitter as rustc_errors[9cfc93648b64d3e2]::emitter::Emitter>::emit_diagnostic
  20:     0x7f673086fde8 - <rustc_errors[9cfc93648b64d3e2]::HandlerInner>::emit_diagnostic
  21:     0x7f673086bad6 - <rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed as rustc_errors[9cfc93648b64d3e2]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f672fbd89db - <rustc_session[c3bed540ab2d15c6]::parse::ParseSess>::emit_err::<rustc_metadata[f343dc9ddb60eac7]::errors::NonDylibPlugin>
  23:     0x7f672fc17b67 - <rustc_metadata[f343dc9ddb60eac7]::locator::CrateError>::report
  24:     0x7f672fc176a2 - rustc_metadata[f343dc9ddb60eac7]::locator::find_plugin_registrar
  25:     0x7f672fb96f54 - rustc_plugin_impl[415ac189aa4afc62]::load::load_plugins
  26:     0x7f672dca6861 - <rustc_session[c3bed540ab2d15c6]::session::Session>::time::<alloc[d88452978c518c2d]::vec::Vec<for<'a, 'b> fn(&'a mut rustc_plugin_impl[415ac189aa4afc62]::Registry<'b>)>, rustc_interface[b2a467b3995992ac]::passes::register_plugins<&dyn for<'a, 'b> core[28ce999ed76db7a5]::ops::function::Fn<(&'a rustc_session[c3bed540ab2d15c6]::session::Session, &'b mut rustc_lint[45e06d0131c74067]::context::LintStore), Output = ()> + core[28ce999ed76db7a5]::marker::Sync + core[28ce999ed76db7a5]::marker::Send>::{closure#2}>
  27:     0x7f672dc60d21 - rustc_interface[b2a467b3995992ac]::passes::register_plugins::<&dyn for<'a, 'b> core[28ce999ed76db7a5]::ops::function::Fn<(&'a rustc_session[c3bed540ab2d15c6]::session::Session, &'b mut rustc_lint[45e06d0131c74067]::context::LintStore), Output = ()> + core[28ce999ed76db7a5]::marker::Sync + core[28ce999ed76db7a5]::marker::Send>
  28:     0x7f672dc474a7 - <rustc_interface[b2a467b3995992ac]::queries::Queries>::register_plugins
  29:     0x7f672db2c899 - <rustc_interface[b2a467b3995992ac]::interface::Compiler>::enter::<rustc_driver[b81b286ec3c48051]::run_compiler::{closure#1}::{closure#2}, core[28ce999ed76db7a5]::result::Result<core[28ce999ed76db7a5]::option::Option<rustc_interface[b2a467b3995992ac]::queries::Linker>, rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>>
  30:     0x7f672db107e1 - rustc_span[3d627267eb982dc3]::with_source_map::<core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>, rustc_interface[b2a467b3995992ac]::interface::create_compiler_and_run<core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>, rustc_driver[b81b286ec3c48051]::run_compiler::{closure#1}>::{closure#1}>
  31:     0x7f672db484a1 - rustc_interface[b2a467b3995992ac]::interface::create_compiler_and_run::<core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>, rustc_driver[b81b286ec3c48051]::run_compiler::{closure#1}>
  32:     0x7f672db083b2 - <scoped_tls[9ff0055e18fa3f5]::ScopedKey<rustc_span[3d627267eb982dc3]::SessionGlobals>>::set::<rustc_interface[b2a467b3995992ac]::interface::run_compiler<core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>, rustc_driver[b81b286ec3c48051]::run_compiler::{closure#1}>::{closure#0}, core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>>
  33:     0x7f672db14129 - std[b633f81169969fc1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b2a467b3995992ac]::util::run_in_thread_pool_with_globals<rustc_interface[b2a467b3995992ac]::interface::run_compiler<core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>, rustc_driver[b81b286ec3c48051]::run_compiler::{closure#1}>::{closure#0}, core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>>::{closure#0}, core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>>
  34:     0x7f672db8ed7e - std[b633f81169969fc1]::panicking::try::<core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>, core[28ce999ed76db7a5]::panic::unwind_safe::AssertUnwindSafe<<std[b633f81169969fc1]::thread::Builder>::spawn_unchecked_<rustc_interface[b2a467b3995992ac]::util::run_in_thread_pool_with_globals<rustc_interface[b2a467b3995992ac]::interface::run_compiler<core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>, rustc_driver[b81b286ec3c48051]::run_compiler::{closure#1}>::{closure#0}, core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>>::{closure#0}, core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f672db89ef0 - <<std[b633f81169969fc1]::thread::Builder>::spawn_unchecked_<rustc_interface[b2a467b3995992ac]::util::run_in_thread_pool_with_globals<rustc_interface[b2a467b3995992ac]::interface::run_compiler<core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>, rustc_driver[b81b286ec3c48051]::run_compiler::{closure#1}>::{closure#0}, core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>>::{closure#0}, core[28ce999ed76db7a5]::result::Result<(), rustc_errors[9cfc93648b64d3e2]::ErrorGuaranteed>>::{closure#1} as core[28ce999ed76db7a5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f672d1abb75 - std::sys::unix::thread::Thread::new::thread_start::h7c907523f323a540
  37:     0x7f672cf49b43 - <unknown>
  38:     0x7f672cfdba00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (9ea998d2d 2022-08-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

