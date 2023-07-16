
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:19 ~ bad[1af5]::main) (NoSolution): could not prove Binder(TraitPredicate(<isize as Copy>, polarity:Positive), [])
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:149:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1347:13
stack backtrace:
   0:     0x7fd8bc29defd - std::backtrace_rs::backtrace::libunwind::trace::h8b2f13ca15763657
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd8bc29defd - std::backtrace_rs::backtrace::trace_unsynchronized::h27431a03bc396c35
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd8bc29defd - std::sys_common::backtrace::_print_fmt::h85c4be4815ee95d0
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fd8bc29defd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb4d052801e236c20
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fd8bc2f9c7c - core::fmt::write::h2baa2cf7fbb66648
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/core/src/fmt/mod.rs:1194:17
   5:     0x7fd8bc28f691 - std::io::Write::write_fmt::hee014219a0480f85
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/io/mod.rs:1655:15
   6:     0x7fd8bc2a0c15 - std::sys_common::backtrace::_print::h16e6cb4501e5ce4b
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fd8bc2a0c15 - std::sys_common::backtrace::print::h3b91c0a9f6fba3d1
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fd8bc2a0c15 - std::panicking::default_hook::{{closure}}::he9e05bbbc618627c
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/panicking.rs:295:22
   9:     0x7fd8bc2a0889 - std::panicking::default_hook::hc1d2a0dd98be282b
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/panicking.rs:314:9
  10:     0x7fd8bcacc4d1 - rustc_driver[c5504433ef6ae1e]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd8bc2a13e6 - std::panicking::rust_panic_with_hook::h4f74f0cdd6986440
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/panicking.rs:702:17
  12:     0x7fd8bdc10b01 - std[2512581f16bcafad]::panicking::begin_panic::<rustc_errors[35505590feeea3b9]::ExplicitBug>::{closure#0}
  13:     0x7fd8bdc0f9d6 - std[2512581f16bcafad]::sys_common::backtrace::__rust_end_short_backtrace::<std[2512581f16bcafad]::panicking::begin_panic<rustc_errors[35505590feeea3b9]::ExplicitBug>::{closure#0}, !>
  14:     0x7fd8bdc2807f - std[2512581f16bcafad]::panicking::begin_panic::<rustc_errors[35505590feeea3b9]::ExplicitBug>
  15:     0x7fd8bdc24d36 - std[2512581f16bcafad]::panic::panic_any::<rustc_errors[35505590feeea3b9]::ExplicitBug>
  16:     0x7fd8bf3cc097 - <rustc_errors[35505590feeea3b9]::HandlerInner as core[4c7b0892081a9ca1]::ops::drop::Drop>::drop
  17:     0x7fd8beb0bf68 - core[4c7b0892081a9ca1]::ptr::drop_in_place::<rustc_session[730db1894ed7a625]::parse::ParseSess>
  18:     0x7fd8beb0e873 - <alloc[f70441f5864a1b32]::rc::Rc<rustc_session[730db1894ed7a625]::session::Session> as core[4c7b0892081a9ca1]::ops::drop::Drop>::drop
  19:     0x7fd8beb0b5ed - core[4c7b0892081a9ca1]::ptr::drop_in_place::<rustc_interface[cc3c07523dc95d8e]::interface::Compiler>
  20:     0x7fd8beb0aee4 - rustc_span[a90ae992492d00db]::with_source_map::<core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>, rustc_interface[cc3c07523dc95d8e]::interface::create_compiler_and_run<core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>, rustc_driver[c5504433ef6ae1e]::run_compiler::{closure#1}>::{closure#1}>
  21:     0x7fd8beaf4c94 - rustc_interface[cc3c07523dc95d8e]::interface::create_compiler_and_run::<core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>, rustc_driver[c5504433ef6ae1e]::run_compiler::{closure#1}>
  22:     0x7fd8beade8d1 - <scoped_tls[59d43b406989f98d]::ScopedKey<rustc_span[a90ae992492d00db]::SessionGlobals>>::set::<rustc_interface[cc3c07523dc95d8e]::interface::run_compiler<core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>, rustc_driver[c5504433ef6ae1e]::run_compiler::{closure#1}>::{closure#0}, core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>>
  23:     0x7fd8beaf7e6f - std[2512581f16bcafad]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cc3c07523dc95d8e]::util::run_in_thread_pool_with_globals<rustc_interface[cc3c07523dc95d8e]::interface::run_compiler<core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>, rustc_driver[c5504433ef6ae1e]::run_compiler::{closure#1}>::{closure#0}, core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>>::{closure#0}, core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>>
  24:     0x7fd8beaf7fb9 - <<std[2512581f16bcafad]::thread::Builder>::spawn_unchecked_<rustc_interface[cc3c07523dc95d8e]::util::run_in_thread_pool_with_globals<rustc_interface[cc3c07523dc95d8e]::interface::run_compiler<core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>, rustc_driver[c5504433ef6ae1e]::run_compiler::{closure#1}>::{closure#0}, core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>>::{closure#0}, core[4c7b0892081a9ca1]::result::Result<(), rustc_errors[35505590feeea3b9]::ErrorGuaranteed>>::{closure#1} as core[4c7b0892081a9ca1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7fd8bc2ab333 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb7ad772b378c0600
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/alloc/src/boxed.rs:1866:9
  26:     0x7fd8bc2ab333 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4f8a3b9480e6ce6e
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/alloc/src/boxed.rs:1866:9
  27:     0x7fd8bc2ab333 - std::sys::unix::thread::Thread::new::thread_start::h2cc17b2e6ab55627
                               at /rustc/b21759f5509477522a208b27bec5822d89f7c6b8/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7fd8bc0835c2 - start_thread
  29:     0x7fd8bc108584 - __clone
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (b21759f55 2022-04-24) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
