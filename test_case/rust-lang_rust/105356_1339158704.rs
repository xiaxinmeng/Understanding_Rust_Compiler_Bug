plain
.................i...................................................................... 3960/3985
.........................
failures:

---- src/intrinsics/mir.rs - intrinsics::mir (line 60) stdout ----
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:8 ~ rust_out[8e0b]::main::_doctest_main_src_intrinsics_mir_rs_60_0::unwrap_unchecked) (NoSolution): could not prove Binder(TraitPredicate(<T as std::marker::Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1610:13
stack backtrace:
stack backtrace:
   0:     0x7fe4fed486e2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h52624f48807f6cde
   1:     0x7fe4fedb66f8 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7fe4fed391b1 - std::io::Write::write_fmt::hbb7597a699362fc1
   3:     0x7fe4fed484a5 - std::sys_common::backtrace::print::h697c84e710d2c4b9
   4:     0x7fe4fed4b7d7 - std::panicking::default_hook::{{closure}}::h4a478635e75d5f67
   5:     0x7fe4fed4b535 - std::panicking::default_hook::hd3d57561af1e5c6e
   6:     0x7fe4ff7392d4 - rustc_driver[8485e792b90a0afe]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe4fed4c0e3 - std::panicking::rust_panic_with_hook::h777a2bf7e3191080
   8:     0x7fe50275eff3 - std[1cf6762bb9e78b99]::panicking::begin_panic::<rustc_errors[2cd35ab4149632b1]::ExplicitBug>::{closure#0}
   9:     0x7fe50275acc6 - std[1cf6762bb9e78b99]::sys_common::backtrace::__rust_end_short_backtrace::<std[1cf6762bb9e78b99]::panicking::begin_panic<rustc_errors[2cd35ab4149632b1]::ExplicitBug>::{closure#0}, !>
  10:     0x7fe4ff6ffda6 - std[1cf6762bb9e78b99]::panicking::begin_panic::<rustc_errors[2cd35ab4149632b1]::ExplicitBug>
  11:     0x7fe5027c1976 - std[1cf6762bb9e78b99]::panic::panic_any::<rustc_errors[2cd35ab4149632b1]::ExplicitBug>
  12:     0x7fe5027cbd49 - <rustc_errors[2cd35ab4149632b1]::HandlerInner>::flush_delayed::<alloc[6d250edaf69a7784]::vec::Vec<rustc_errors[2cd35ab4149632b1]::diagnostic::Diagnostic>, &str>
  13:     0x7fe5027ca396 - <rustc_errors[2cd35ab4149632b1]::Handler>::flush_delayed
  14:     0x7fe4ff87f371 - <rustc_interface[cd2c8619ab6be845]::passes::QueryContext>::enter::<<rustc_interface[cd2c8619ab6be845]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<alloc[6d250edaf69a7784]::boxed::Box<dyn core[867cfca19013d5a]::any::Any>, rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>
  15:     0x7fe4ff897076 - <rustc_interface[cd2c8619ab6be845]::queries::Queries>::ongoing_codegen
  16:     0x7fe4ff79af9e - <rustc_interface[cd2c8619ab6be845]::interface::Compiler>::enter::<rustc_driver[8485e792b90a0afe]::run_compiler::{closure#1}::{closure#2}, core[867cfca19013d5a]::result::Result<core[867cfca19013d5a]::option::Option<rustc_interface[cd2c8619ab6be845]::queries::Linker>, rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>
  17:     0x7fe4ff73a986 - rustc_span[a8da9c41bdabf00d]::with_source_map::<core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>, rustc_interface[cd2c8619ab6be845]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>, rustc_driver[8485e792b90a0afe]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  18:     0x7fe4ff79ba48 - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[a8da9c41bdabf00d]::SessionGlobals>>::set::<rustc_interface[cd2c8619ab6be845]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>, rustc_driver[8485e792b90a0afe]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>
  19:     0x7fe4ff757f20 - std[1cf6762bb9e78b99]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cd2c8619ab6be845]::util::run_in_thread_pool_with_globals<rustc_interface[cd2c8619ab6be845]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>, rustc_driver[8485e792b90a0afe]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>
  20:     0x7fe4ff79cdf6 - std[1cf6762bb9e78b99]::panicking::try::<core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>, core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<<std[1cf6762bb9e78b99]::thread::Builder>::spawn_unchecked_<rustc_interface[cd2c8619ab6be845]::util::run_in_thread_pool_with_globals<rustc_interface[cd2c8619ab6be845]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>, rustc_driver[8485e792b90a0afe]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  21:     0x7fe4ff74aad9 - <<std[1cf6762bb9e78b99]::thread::Builder>::spawn_unchecked_<rustc_interface[cd2c8619ab6be845]::util::run_in_thread_pool_with_globals<rustc_interface[cd2c8619ab6be845]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>, rustc_driver[8485e792b90a0afe]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[2cd35ab4149632b1]::ErrorGuaranteed>>::{closure#1} as core[867cfca19013d5a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7fe4fed58ffe - std::sys::unix::thread::Thread::new::thread_start::h454bcc72a7f69298
  23:     0x7fe4feaedb43 - <unknown>
  24:     0x7fe4feb7fa00 - <unknown>
  25:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (3d28af116 2022-12-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -Z unstable-options -C codegen-units=1 -C embed-bitcode=no -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z unstable-options -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z force-unstable-if-unmarked -Z unstable-options
query stack during panic:
end of query stack
error: aborting due to 2 previous errors


Couldn't compile the test.

failures:
    src/intrinsics/mir.rs - intrinsics::mir (line 60)

test result: FAILED. 3948 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 69.19s

error: doctest failed, to rerun pass `-p core --doc`
