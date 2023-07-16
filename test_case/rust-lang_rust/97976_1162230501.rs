
$ rustdoc +nightly src/main.rs -Ztreat-err-as-bug
error[E0433]: failed to resolve: maybe a missing crate `winapi`?
 --> src/main.rs:3:9
  |
3 |     use winapi::um::debugapi::OutputDebugStringW;
  |         ^^^^^^ maybe a missing crate `winapi`?
  |
  = help: consider adding `extern crate winapi` to use the `winapi` crate

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1449:27
stack backtrace:
   0:     0x7f8cb4a9e49d - std::backtrace_rs::backtrace::libunwind::trace::h38cae73bd792c877
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f8cb4a9e49d - std::backtrace_rs::backtrace::trace_unsynchronized::h8e053040b0b51451
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f8cb4a9e49d - std::sys_common::backtrace::_print_fmt::h31b97fe7e89502f9
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f8cb4a9e49d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5c671a6da31da332
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f8cb4afa2dc - core::fmt::write::h41097386f3da9277
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/core/src/fmt/mod.rs:1196:17
   5:     0x7f8cb4a8fb01 - std::io::Write::write_fmt::hb65c9f86c8aba0db
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/io/mod.rs:1654:15
   6:     0x7f8cb4aa1175 - std::sys_common::backtrace::_print::hdc514304bc245eca
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f8cb4aa1175 - std::sys_common::backtrace::print::h151e88258ea30387
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f8cb4aa1175 - std::panicking::default_hook::{{closure}}::h73c1e663080d3f2d
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/panicking.rs:295:22
   9:     0x7f8cb4aa0e96 - std::panicking::default_hook::h7e38b1d6902bcb35
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/panicking.rs:314:9
  10:     0x7f8cb52fc051 - rustc_driver[a943fb23bb6f3d7]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f8cb4aa184a - std::panicking::rust_panic_with_hook::h61964c46760a3d63
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/panicking.rs:702:17
  12:     0x7f8cb4aa1649 - std::panicking::begin_panic_handler::{{closure}}::h30113bf9b3b03d33
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/panicking.rs:586:13
  13:     0x7f8cb4a9e954 - std::sys_common::backtrace::__rust_end_short_backtrace::h62a1f4cef7db596d
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f8cb4aa13b9 - rust_begin_unwind
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/panicking.rs:584:5
  15:     0x7f8cb4a662c3 - core::panicking::panic_fmt::h67caabfe06301e98
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/core/src/panicking.rs:142:14
  16:     0x7f8cb64a0dde - <rustc_errors[6bc7ce1ecb76dda5]::HandlerInner>::panic_if_treat_err_as_bug
  17:     0x7f8cb7d64699 - <rustc_errors[6bc7ce1ecb76dda5]::HandlerInner>::emit_diagnostic
  18:     0x7f8cb7d63c01 - <rustc_errors[6bc7ce1ecb76dda5]::Handler>::emit_diagnostic
  19:     0x7f8cb649fff9 - <rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed as rustc_errors[6bc7ce1ecb76dda5]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  20:     0x7f8cb575fd98 - <rustc_resolve[8675f4fc89cebd7c]::Resolver>::report_error
  21:     0x7f8cb779a3a8 - <rustc_resolve[8675f4fc89cebd7c]::imports::ImportResolver>::finalize_import
  22:     0x7f8cb7797e23 - <rustc_resolve[8675f4fc89cebd7c]::imports::ImportResolver>::finalize_imports
  23:     0x7f8cb779bd7b - <rustc_session[1f78975236c951ca]::session::Session>::time::<(), <rustc_resolve[8675f4fc89cebd7c]::Resolver>::resolve_crate::{closure#0}>
  24:     0x7f8cb74f018a - rustc_interface[230b39356bc6dc4e]::passes::configure_and_expand
  25:     0x7f8cb74e8863 - <rustc_interface[230b39356bc6dc4e]::queries::Queries>::expansion
  26:     0x561e71e2eaa0 - <rustc_interface[230b39356bc6dc4e]::interface::Compiler>::enter::<rustdoc[41ddfdff4ee9a4db]::main_options::{closure#0}::{closure#0}, core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>>
  27:     0x561e71cde33a - rustc_span[2a7938c1129556cc]::with_source_map::<core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>, rustc_interface[230b39356bc6dc4e]::interface::create_compiler_and_run<core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>, rustdoc[41ddfdff4ee9a4db]::main_options::{closure#0}>::{closure#1}>
  28:     0x561e71e312a1 - rustc_interface[230b39356bc6dc4e]::interface::create_compiler_and_run::<core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>, rustdoc[41ddfdff4ee9a4db]::main_options::{closure#0}>
  29:     0x561e71ce267f - <scoped_tls[9b0a97b116fbaba]::ScopedKey<rustc_span[2a7938c1129556cc]::SessionGlobals>>::set::<rustdoc[41ddfdff4ee9a4db]::main_args::{closure#0}, core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>>
  30:     0x561e71e5933f - std[453ac8ab9b5fa23e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[230b39356bc6dc4e]::util::run_in_thread_pool_with_globals<rustdoc[41ddfdff4ee9a4db]::main_args::{closure#0}, core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>>::{closure#0}, core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>>
  31:     0x561e71f49359 - <<std[453ac8ab9b5fa23e]::thread::Builder>::spawn_unchecked_<rustc_interface[230b39356bc6dc4e]::util::run_in_thread_pool_with_globals<rustdoc[41ddfdff4ee9a4db]::main_args::{closure#0}, core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>>::{closure#0}, core[4f3677c414e9e7a2]::result::Result<(), rustc_errors[6bc7ce1ecb76dda5]::ErrorGuaranteed>>::{closure#1} as core[4f3677c414e9e7a2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f8cb4aab783 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha95224ff040ea8fe
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/alloc/src/boxed.rs:1951:9
  33:     0x7f8cb4aab783 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h2429363f7b78f055
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/alloc/src/boxed.rs:1951:9
  34:     0x7f8cb4aab783 - std::sys::unix::thread::Thread::new::thread_start::h00f780f864797841
                               at /rustc/cacc75c82ebe15cf63d31034fcf7f1016cddf0e2/library/std/src/sys/unix/thread.rs:108:17
  35:     0x7f8cb488054d - <unknown>
  36:     0x7f8cb4905874 - clone
  37:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (cacc75c82 2022-06-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug

query stack during panic:
end of query stack
