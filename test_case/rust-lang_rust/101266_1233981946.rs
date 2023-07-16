plain

running 229 tests
..........i.....ii...................................................................... 88/229
..i.........................i...................iiiiiii......i...................iii.... 176/229
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...............................i...ii..F.............

---- [run-make] src/test/run-make-fulldeps/weird-output-filenames stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
cp foo.rs /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/weird-output-filenames/weird-output-filenames/.foo.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/weird-output-filenames/weird-output-filenames:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/weird-output-filenames/weird-output-filenames -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/weird-output-filenames/weird-output-filenames  /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/weird-output-filenames/weird-output-filenames/.foo.rs 2>&1 \
 | "/checkout/src/etc/cat-and-grep.sh" -e "invalid character.*in crate name:"
[[[ begin stdout ]]]
thread 'rustc' panicked at 'identifier: "session_invalid_character_in_create_name", attr: None, args: FluentArgs([]), errors: [ResolverError(Reference(Variable { id: "character" })), ResolverError(Reference(Variable { id: "crate_name" }))]', compiler/rustc_errors/src/translation.rs:91:17
stack backtrace:
   0:     0x7f6b41b5012c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc487dcf423f7dc5
   1:     0x7f6b41bb8ca8 - core::fmt::write::h8b3d9a117ecda149
   2:     0x7f6b41b409a1 - std::io::Write::write_fmt::hc1f8e5607fcae684
   3:     0x7f6b41b5311e - std::panicking::default_hook::{{closure}}::h10b8ec90f5d1420b
   4:     0x7f6b41b52de7 - std::panicking::default_hook::h077c4fc56990cab2
   5:     0x7f6b424e2054 - rustc_driver[939608cf3f9c8401]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6b41b538d1 - std::panicking::rust_panic_with_hook::h255d1fc3fdfe100d
   7:     0x7f6b41b536f7 - std::panicking::begin_panic_handler::{{closure}}::hbe73b989bc6b0e39
   8:     0x7f6b41b506a4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb90e20907a7a19fd
   9:     0x7f6b41b533c2 - rust_begin_unwind
  10:     0x7f6b41b03e43 - core::panicking::panic_fmt::h206079c3bf5fa5e3
  11:     0x7f6b451d8a6a - <rustc_errors[80a7ddfbd784b988]::emitter::EmitterWriter as rustc_errors[80a7ddfbd784b988]::translation::Translate>::translate_message
  12:     0x7f6b451ce052 - <rustc_errors[80a7ddfbd784b988]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f6b451cbb72 - <rustc_errors[80a7ddfbd784b988]::emitter::EmitterWriter as rustc_errors[80a7ddfbd784b988]::emitter::Emitter>::emit_diagnostic
  14:     0x7f6b452258f8 - <rustc_errors[80a7ddfbd784b988]::HandlerInner>::emit_diagnostic
  15:     0x7f6b452215a6 - <rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed as rustc_errors[80a7ddfbd784b988]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  16:     0x7f6b4514a21f - <rustc_session[acee3a4a0a70eadd]::parse::ParseSess>::emit_err::<rustc_session[acee3a4a0a70eadd]::errors::InvalidCharacterInCrateName>
  17:     0x7f6b451a8a34 - rustc_session[acee3a4a0a70eadd]::output::validate_crate_name
  18:     0x7f6b451a8866 - rustc_session[acee3a4a0a70eadd]::output::find_crate_name
  19:     0x7f6b4261b544 - <rustc_interface[be3e3b91a6472b5b]::queries::Queries>::crate_name
  20:     0x7f6b4261adca - <rustc_interface[be3e3b91a6472b5b]::queries::Queries>::register_plugins
  21:     0x7f6b424e3792 - <rustc_interface[be3e3b91a6472b5b]::interface::Compiler>::enter::<rustc_driver[939608cf3f9c8401]::run_compiler::{closure#1}::{closure#2}, core[e1c75ed3987e2883]::result::Result<core[e1c75ed3987e2883]::option::Option<rustc_interface[be3e3b91a6472b5b]::queries::Linker>, rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>>
  22:     0x7f6b424d0551 - rustc_span[6fde1357d18f6af6]::with_source_map::<core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>, rustc_interface[be3e3b91a6472b5b]::interface::create_compiler_and_run<core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>, rustc_driver[939608cf3f9c8401]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x7f6b424e6661 - rustc_interface[be3e3b91a6472b5b]::interface::create_compiler_and_run::<core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>, rustc_driver[939608cf3f9c8401]::run_compiler::{closure#1}>
  24:     0x7f6b424cd3c2 - <scoped_tls[723eb8456d42f204]::ScopedKey<rustc_span[6fde1357d18f6af6]::SessionGlobals>>::set::<rustc_interface[be3e3b91a6472b5b]::interface::run_compiler<core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>, rustc_driver[939608cf3f9c8401]::run_compiler::{closure#1}>::{closure#0}, core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>>
  25:     0x7f6b4254554f - std[3417fb0cb2b602d0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[be3e3b91a6472b5b]::util::run_in_thread_pool_with_globals<rustc_interface[be3e3b91a6472b5b]::interface::run_compiler<core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>, rustc_driver[939608cf3f9c8401]::run_compiler::{closure#1}>::{closure#0}, core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>>::{closure#0}, core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>>
  26:     0x7f6b424cd9ae - std[3417fb0cb2b602d0]::panicking::try::<core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>, core[e1c75ed3987e2883]::panic::unwind_safe::AssertUnwindSafe<<std[3417fb0cb2b602d0]::thread::Builder>::spawn_unchecked_<rustc_interface[be3e3b91a6472b5b]::util::run_in_thread_pool_with_globals<rustc_interface[be3e3b91a6472b5b]::interface::run_compiler<core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>, rustc_driver[939608cf3f9c8401]::run_compiler::{closure#1}>::{closure#0}, core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>>::{closure#0}, core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  27:     0x7f6b42548930 - <<std[3417fb0cb2b602d0]::thread::Builder>::spawn_unchecked_<rustc_interface[be3e3b91a6472b5b]::util::run_in_thread_pool_with_globals<rustc_interface[be3e3b91a6472b5b]::interface::run_compiler<core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>, rustc_driver[939608cf3f9c8401]::run_compiler::{closure#1}>::{closure#0}, core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>>::{closure#0}, core[e1c75ed3987e2883]::result::Result<(), rustc_errors[80a7ddfbd784b988]::ErrorGuaranteed>>::{closure#1} as core[e1c75ed3987e2883]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  28:     0x7f6b41b60645 - std::sys::unix::thread::Thread::new::thread_start::h1e1f1330b83c15f9
  29:     0x7f6b418fcb43 - <unknown>
  30:     0x7f6b4198ea00 - <unknown>
  31:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (e34cc674c 2022-09-01) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack

[[[ end stdout ]]]
Error: cannot match: invalid character.*in crate name:
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:5: all] Error 1



failures:
