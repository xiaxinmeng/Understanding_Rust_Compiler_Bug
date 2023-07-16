plain
..............................................iii....................................... 13376/13441
.................................................................
failures:

---- [ui] src/test/ui/unpretty/avoid-crash.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unpretty/avoid-crash.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/avoid-crash" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-o/tmp/" "-Zunpretty=ast-tree" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unpretty/avoid-crash/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'identifier: "driver_unpretty_dump_fail", attr: None, args: FluentArgs([("err", String("Is a directory (os error 21)")), ("path", String("/tmp/"))]), errors: [ResolverError(Reference(Variable { id: "file" }))]', compiler/rustc_errors/src/translation.rs:91:17
   0:     0x7f994c6bccfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2d70728efb7f17c4
   1:     0x7f994c725748 - core::fmt::write::h164d8178aacd45b9
   2:     0x7f994c6ad8d1 - std::io::Write::write_fmt::h7441a3e474742736
   2:     0x7f994c6ad8d1 - std::io::Write::write_fmt::h7441a3e474742736
   3:     0x7f994c6bfcfe - std::panicking::default_hook::{{closure}}::he4d279a320ac4af6
   4:     0x7f994c6bf9c7 - std::panicking::default_hook::haacc8308ff9c5b22
   5:     0x7f994d0461d4 - rustc_driver[5aef8eeba772cc36]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f994c6c04b1 - std::panicking::rust_panic_with_hook::hd0d6dcb471afe330
   7:     0x7f994c6c02d7 - std::panicking::begin_panic_handler::{{closure}}::he4ed78d399c7708c
   8:     0x7f994c6bd274 - std::sys_common::backtrace::__rust_end_short_backtrace::h3239e9adb204c6ee
   9:     0x7f994c6bffa2 - rust_begin_unwind
  10:     0x7f994c670e43 - core::panicking::panic_fmt::h14671cead6fd95e8
  11:     0x7f994fd3d697 - <rustc_errors[686c94903a87aef8]::emitter::EmitterWriter as rustc_errors[686c94903a87aef8]::translation::Translate>::translate_message
  12:     0x7f994fd32a12 - <rustc_errors[686c94903a87aef8]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f994fd304b6 - <rustc_errors[686c94903a87aef8]::emitter::EmitterWriter as rustc_errors[686c94903a87aef8]::emitter::Emitter>::emit_diagnostic
  14:     0x7f994fd48c52 - <rustc_errors[686c94903a87aef8]::json::Diagnostic>::from_errors_diagnostic
  15:     0x7f994fd4766c - <rustc_errors[686c94903a87aef8]::json::JsonEmitter as rustc_errors[686c94903a87aef8]::emitter::Emitter>::emit_diagnostic
  16:     0x7f994fd8d338 - <rustc_errors[686c94903a87aef8]::HandlerInner>::emit_diagnostic
  17:     0x7f994fd8bc31 - <rustc_errors[686c94903a87aef8]::Handler>::emit_diagnostic
  18:     0x7f994fd942fd - <! as rustc_errors[686c94903a87aef8]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  19:     0x7f994d0358f7 - <rustc_errors[686c94903a87aef8]::diagnostic_builder::DiagnosticBuilder<!>>::emit
  20:     0x7f994d0b1145 - <rustc_session[f1b3e05090da4f2d]::parse::ParseSess>::emit_fatal::<rustc_driver[5aef8eeba772cc36]::session_diagnostics::UnprettyDumpFail>
  21:     0x7f994d04ec1c - <rustc_session[f1b3e05090da4f2d]::session::Session>::emit_fatal::<rustc_driver[5aef8eeba772cc36]::session_diagnostics::UnprettyDumpFail>
  22:     0x7f994d0ba585 - rustc_driver[5aef8eeba772cc36]::pretty::write_or_print
  23:     0x7f994d0ba98e - rustc_driver[5aef8eeba772cc36]::pretty::print_after_parsing
  24:     0x7f994d04e005 - <rustc_interface[d70dfd52df333278]::interface::Compiler>::enter::<rustc_driver[5aef8eeba772cc36]::run_compiler::{closure#1}::{closure#2}, core[4b23ab53ab65e410]::result::Result<core[4b23ab53ab65e410]::option::Option<rustc_interface[d70dfd52df333278]::queries::Linker>, rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>
  25:     0x7f994d030f85 - rustc_span[ac2dfba6ecd27895]::with_source_map::<core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>, rustc_interface[d70dfd52df333278]::interface::create_compiler_and_run<core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>, rustc_driver[5aef8eeba772cc36]::run_compiler::{closure#1}>::{closure#1}>
  26:     0x7f994d069c11 - rustc_interface[d70dfd52df333278]::interface::create_compiler_and_run::<core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>, rustc_driver[5aef8eeba772cc36]::run_compiler::{closure#1}>
  27:     0x7f994d032582 - <scoped_tls[6596881b5accfbf6]::ScopedKey<rustc_span[ac2dfba6ecd27895]::SessionGlobals>>::set::<rustc_interface[d70dfd52df333278]::interface::run_compiler<core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>, rustc_driver[5aef8eeba772cc36]::run_compiler::{closure#1}>::{closure#0}, core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>
  28:     0x7f994d0a8249 - std[a2e642b7c288af67]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d70dfd52df333278]::util::run_in_thread_pool_with_globals<rustc_interface[d70dfd52df333278]::interface::run_compiler<core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>, rustc_driver[5aef8eeba772cc36]::run_compiler::{closure#1}>::{closure#0}, core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>::{closure#0}, core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>
  29:     0x7f994d034bae - std[a2e642b7c288af67]::panic::catch_unwind::<core[4b23ab53ab65e410]::panic::unwind_safe::AssertUnwindSafe<<std[a2e642b7c288af67]::thread::Builder>::spawn_unchecked_<rustc_interface[d70dfd52df333278]::util::run_in_thread_pool_with_globals<rustc_interface[d70dfd52df333278]::interface::run_compiler<core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>, rustc_driver[5aef8eeba772cc36]::run_compiler::{closure#1}>::{closure#0}, core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>::{closure#0}, core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>
  30:     0x7f994d0ac890 - <<std[a2e642b7c288af67]::thread::Builder>::spawn_unchecked_<rustc_interface[d70dfd52df333278]::util::run_in_thread_pool_with_globals<rustc_interface[d70dfd52df333278]::interface::run_compiler<core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>, rustc_driver[5aef8eeba772cc36]::run_compiler::{closure#1}>::{closure#0}, core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>::{closure#0}, core[4b23ab53ab65e410]::result::Result<(), rustc_errors[686c94903a87aef8]::ErrorGuaranteed>>::{closure#1} as core[4b23ab53ab65e410]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f994c6ccd25 - std::sys::unix::thread::Thread::new::thread_start::hc73c0c48f1303513
  32:     0x7f994c46ab43 - <unknown>
  33:     0x7f994c4fca00 - <unknown>
  34:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (093093ff5 2022-08-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z unpretty=ast-tree
query stack during panic:
end of query stack
------------------------------------------

