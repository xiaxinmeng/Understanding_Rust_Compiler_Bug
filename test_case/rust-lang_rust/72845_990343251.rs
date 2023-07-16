`
warning: trait method `Bar` should have a snake case name
  --> ./72845.rs:36:8
   |
36 |     fn Bar();
   |        ^^^ help: convert the identifier to snake case: `bar`
   |
   = note: `#[warn(non_snake_case)]` on by default

warning: 1 warning emitted

error: internal compiler error: failed to normalize <<T as Type>::AT as Depth>::C
  |
  = note: delayed at compiler/rustc_const_eval/src/interpret/eval_context.rs:527:31

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1189:13
stack backtrace:
   0:     0x7f17cb4cc35c - std::backtrace_rs::backtrace::libunwind::trace::hf7449935ead7573e
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f17cb4cc35c - std::backtrace_rs::backtrace::trace_unsynchronized::h221aa2d88d72372a
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f17cb4cc35c - std::sys_common::backtrace::_print_fmt::h1c77e8983e1df895
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f17cb4cc35c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd4ec41a9a6b0d22c
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f17cb52937c - core::fmt::write::h72801a82c94e6ff1
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/core/src/fmt/mod.rs:1149:17
   5:     0x7f17cb4bca65 - std::io::Write::write_fmt::haf74340a8cbeaa88
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/io/mod.rs:1660:15
   6:     0x7f17cb4cf520 - std::sys_common::backtrace::_print::h2d15cd157796a64a
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f17cb4cf520 - std::sys_common::backtrace::print::h52d286d22e2398eb
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f17cb4cf520 - std::panicking::default_hook::{{closure}}::h6da08fba6306daf2
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:211:50
   9:     0x7f17cb4cf0cb - std::panicking::default_hook::h266f67a22e76b11a
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:228:9
  10:     0x7f17cbff3291 - rustc_driver[feb513e6adc957d8]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f17cb4cfd39 - std::panicking::rust_panic_with_hook::he55698a957f4fb6d
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:610:17
  12:     0x7f17cb4cf7f0 - std::panicking::begin_panic_handler::{{closure}}::h01f453c3ac181895
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:502:13
  13:     0x7f17cb4cc804 - std::sys_common::backtrace::__rust_end_short_backtrace::h675d77c6e5a3926d
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys_common/backtrace.rs:139:18
  14:     0x7f17cb4cf759 - rust_begin_unwind
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/panicking.rs:498:5
  15:     0x7f17cb493c21 - core::panicking::panic_fmt::h7b8580d81fcbbacd
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/core/src/panicking.rs:107:14
  16:     0x7f17cd12fe9c - core[cc79c391059f8e46]::panicking::panic_display::<&str>
  17:     0x7f17ce7c4e7a - <rustc_errors[248f3f311c690252]::HandlerInner>::flush_delayed
  18:     0x7f17ce7c35fd - <rustc_errors[248f3f311c690252]::HandlerInner as core[cc79c391059f8e46]::ops::drop::Drop>::drop
  19:     0x7f17cdea73b6 - core[cc79c391059f8e46]::ptr::drop_in_place::<rustc_session[2680299e10ec9d86]::parse::ParseSess>
  20:     0x7f17cdea9a9a - <alloc[1954047e53701c4d]::rc::Rc<rustc_session[2680299e10ec9d86]::session::Session> as core[cc79c391059f8e46]::ops::drop::Drop>::drop
  21:     0x7f17cde8496d - core[cc79c391059f8e46]::ptr::drop_in_place::<rustc_interface[4c4b5644b43577f1]::interface::Compiler>
  22:     0x7f17cde84332 - rustc_span[dda57b1885b40b9a]::with_source_map::<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_interface[4c4b5644b43577f1]::interface::create_compiler_and_run<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}>::{closure#1}>
  23:     0x7f17cde887ef - <scoped_tls[3fea4c3dcac147b1]::ScopedKey<rustc_span[dda57b1885b40b9a]::SessionGlobals>>::set::<rustc_interface[4c4b5644b43577f1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[4c4b5644b43577f1]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>::{closure#0}::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>
  24:     0x7f17cde86995 - std[f24903a91e569aa2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4c4b5644b43577f1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[4c4b5644b43577f1]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>
  25:     0x7f17cdea6ffa - <<std[f24903a91e569aa2]::thread::Builder>::spawn_unchecked<rustc_interface[4c4b5644b43577f1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[4c4b5644b43577f1]::interface::run_compiler<core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>, rustc_driver[feb513e6adc957d8]::run_compiler::{closure#1}>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>::{closure#0}, core[cc79c391059f8e46]::result::Result<(), rustc_errors[248f3f311c690252]::ErrorReported>>::{closure#1} as core[cc79c391059f8e46]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x7f17cb4dae23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4beb69a85f7fb16c
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/alloc/src/boxed.rs:1811:9
  27:     0x7f17cb4dae23 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hef865a799f44aaf2
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/alloc/src/boxed.rs:1811:9
  28:     0x7f17cb4dae23 - std::sys::unix::thread::Thread::new::thread_start::hee0b2d4e2414fa96
                               at /rustc/0b42deaccc2cbe17a68067aa5fdb76104369e1fd/library/std/src/sys/unix/thread.rs:108:17
  29:     0x7f17cb3e8259 - start_thread
  30:     0x7f17cb2fd5e3 - __GI___clone
  31:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (0b42deacc 2021-12-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib

query stack during panic:
end of query stack
