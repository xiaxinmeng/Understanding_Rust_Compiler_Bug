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
   0:     0x7f82e105f12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb8b0ea50cbeb31bc
   1:     0x7f82e10c7c68 - core::fmt::write::he1621e459e3f3b44
   2:     0x7f82e104f9a1 - std::io::Write::write_fmt::hf44592a09a5f52a7
   3:     0x7f82e106211e - std::panicking::default_hook::{{closure}}::he9048b9e2c20e213
   4:     0x7f82e1061de7 - std::panicking::default_hook::h69c33f294b050fe4
   5:     0x7f82e1a1d9f4 - rustc_driver[7b0b75ed33f6db93]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f82e10628d1 - std::panicking::rust_panic_with_hook::he6f9faf2f0cc49bc
   7:     0x7f82e10626f7 - std::panicking::begin_panic_handler::{{closure}}::h1de83dda4702ae05
   8:     0x7f82e105f6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::hc4ae1b4b27bcce55
   9:     0x7f82e10623c2 - rust_begin_unwind
  10:     0x7f82e1012e43 - core::panicking::panic_fmt::h8435913bff5af4b2
  11:     0x7f82e477af1a - <rustc_errors[7530e9a6acc689f7]::emitter::EmitterWriter as rustc_errors[7530e9a6acc689f7]::translation::Translate>::translate_message
  12:     0x7f82e4770502 - <rustc_errors[7530e9a6acc689f7]::emitter::EmitterWriter>::emit_message_default
  13:     0x7f82e476e022 - <rustc_errors[7530e9a6acc689f7]::emitter::EmitterWriter as rustc_errors[7530e9a6acc689f7]::emitter::Emitter>::emit_diagnostic
  14:     0x7f82e47c7ce8 - <rustc_errors[7530e9a6acc689f7]::HandlerInner>::emit_diagnostic
  15:     0x7f82e47c3996 - <rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed as rustc_errors[7530e9a6acc689f7]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  16:     0x7f82e46ec56f - <rustc_session[65a812d6fbf18884]::parse::ParseSess>::emit_err::<rustc_session[65a812d6fbf18884]::errors::InvalidCharacterInCrateName>
  17:     0x7f82e4741824 - rustc_session[65a812d6fbf18884]::output::validate_crate_name
  18:     0x7f82e4741618 - rustc_session[65a812d6fbf18884]::output::find_crate_name
  19:     0x7f82e1b58094 - <rustc_interface[74d98f0093b22e39]::queries::Queries>::crate_name
  20:     0x7f82e1b57878 - <rustc_interface[74d98f0093b22e39]::queries::Queries>::register_plugins
  21:     0x7f82e1a25b8a - <rustc_interface[74d98f0093b22e39]::interface::Compiler>::enter::<rustc_driver[7b0b75ed33f6db93]::run_compiler::{closure#1}::{closure#2}, core[9f4fd3e9a55d9ddc]::result::Result<core[9f4fd3e9a55d9ddc]::option::Option<rustc_interface[74d98f0093b22e39]::queries::Linker>, rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>>
  22:     0x7f82e1a0ca11 - rustc_span[d0496a76cabc13cb]::with_source_map::<core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>, rustc_interface[74d98f0093b22e39]::interface::create_compiler_and_run<core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>, rustc_driver[7b0b75ed33f6db93]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x7f82e1a28381 - rustc_interface[74d98f0093b22e39]::interface::create_compiler_and_run::<core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>, rustc_driver[7b0b75ed33f6db93]::run_compiler::{closure#1}>
  24:     0x7f82e1a08912 - <scoped_tls[a3cec3809d4d6e1b]::ScopedKey<rustc_span[d0496a76cabc13cb]::SessionGlobals>>::set::<rustc_interface[74d98f0093b22e39]::interface::run_compiler<core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>, rustc_driver[7b0b75ed33f6db93]::run_compiler::{closure#1}>::{closure#0}, core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>>
  25:     0x7f82e1a81bef - std[68eedc93381c1604]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[74d98f0093b22e39]::util::run_in_thread_pool_with_globals<rustc_interface[74d98f0093b22e39]::interface::run_compiler<core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>, rustc_driver[7b0b75ed33f6db93]::run_compiler::{closure#1}>::{closure#0}, core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>>::{closure#0}, core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>>
  26:     0x7f82e1a0da8e - std[68eedc93381c1604]::panicking::try::<core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>, core[9f4fd3e9a55d9ddc]::panic::unwind_safe::AssertUnwindSafe<<std[68eedc93381c1604]::thread::Builder>::spawn_unchecked_<rustc_interface[74d98f0093b22e39]::util::run_in_thread_pool_with_globals<rustc_interface[74d98f0093b22e39]::interface::run_compiler<core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>, rustc_driver[7b0b75ed33f6db93]::run_compiler::{closure#1}>::{closure#0}, core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>>::{closure#0}, core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  27:     0x7f82e1a833f0 - <<std[68eedc93381c1604]::thread::Builder>::spawn_unchecked_<rustc_interface[74d98f0093b22e39]::util::run_in_thread_pool_with_globals<rustc_interface[74d98f0093b22e39]::interface::run_compiler<core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>, rustc_driver[7b0b75ed33f6db93]::run_compiler::{closure#1}>::{closure#0}, core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>>::{closure#0}, core[9f4fd3e9a55d9ddc]::result::Result<(), rustc_errors[7530e9a6acc689f7]::ErrorGuaranteed>>::{closure#1} as core[9f4fd3e9a55d9ddc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  28:     0x7f82e106f645 - std::sys::unix::thread::Thread::new::thread_start::h2c1839a326925ba3
  29:     0x7f82e0e0bb43 - <unknown>
  30:     0x7f82e0e9da00 - <unknown>
  31:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (7a70348fa 2022-09-02) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack

[[[ end stdout ]]]
Error: cannot match: invalid character.*in crate name:
--- stderr -------------------------------
make: *** [Makefile:5: all] Error 1
------------------------------------------

