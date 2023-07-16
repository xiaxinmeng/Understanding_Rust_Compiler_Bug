`
error[E0381]: borrow of possibly-uninitialized variable: `s`
 --> ./src/test/ui/const-generics/const-generic-default-wont-borrowck.rs:2:26
  |
2 |     let s: &'static str; s.len()
  |                          ^^^^^^^ use of possibly-uninitialized `*s`

thread 'rustc' panicked at '`SaveContext::typeck_results` called outside of body', compiler/rustc_save_analysis/src/dump_visitor.rs:884:26
stack backtrace:
   0:     0x7f607bcc5b6c - std::backtrace_rs::backtrace::libunwind::trace::h7c6a0b6bbcfb15e0
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f607bcc5b6c - std::backtrace_rs::backtrace::trace_unsynchronized::h738fa839c025d88e
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f607bcc5b6c - std::sys_common::backtrace::_print_fmt::h4f98563e03fd77a0
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f607bcc5b6c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4537eb61bdce6f6b
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f607bd2724c - core::fmt::write::he1e8f14e90f89497
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/core/src/fmt/mod.rs:1190:17
   5:     0x7f607bcb5e98 - std::io::Write::write_fmt::ha9d438e87e29cf6e
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/io/mod.rs:1657:15
   6:     0x7f607bcc9ae7 - std::sys_common::backtrace::_print::h587a4663357bac6f
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f607bcc9ae7 - std::sys_common::backtrace::print::h8dbaaebd1b6f9c35
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f607bcc9ae7 - std::panicking::default_hook::{{closure}}::h11ab986bd24269f7
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/panicking.rs:295:22
   9:     0x7f607bcc97af - std::panicking::default_hook::h23b7cddda18cff14
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/panicking.rs:314:9
  10:     0x7f607c454a51 - rustc_driver[57ba6522360aaed]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f607bcca3c5 - std::panicking::rust_panic_with_hook::heed66b7b6ebf7046
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/panicking.rs:702:17
  12:     0x7f607bcca077 - std::panicking::begin_panic_handler::{{closure}}::h4a817fb4518b6940
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/panicking.rs:588:13
  13:     0x7f607bcc6014 - std::sys_common::backtrace::__rust_end_short_backtrace::hcbda452eda268a56
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f607bcc9d79 - rust_begin_unwind
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/panicking.rs:584:5
  15:     0x7f607bc91d03 - core::panicking::panic_fmt::h4d140668741c6a4f
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/core/src/panicking.rs:143:14
  16:     0x7f607bd24041 - core::panicking::panic_display::h5067549a28a2c8ee
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/core/src/panicking.rs:73:5
  17:     0x7f607bd23feb - core::panicking::panic_str::hd9e42391d8817dbd
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/core/src/panicking.rs:56:5
  18:     0x7f607bc91b76 - core::option::expect_failed::hbec3505e25f361c9
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/core/src/option.rs:1840:5
  19:     0x7f607c47cb30 - <rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor>::process_var_decl
  20:     0x7f607c483bf2 - <rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor as rustc_hir[7415ed71242d56dd]::intravisit::Visitor>::visit_local
  21:     0x7f607c467bb7 - rustc_hir[7415ed71242d56dd]::intravisit::walk_block::<rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor>
  22:     0x7f607c4845f5 - <rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor as rustc_hir[7415ed71242d56dd]::intravisit::Visitor>::visit_nested_body
  23:     0x7f607c481f4e - <rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor as rustc_hir[7415ed71242d56dd]::intravisit::Visitor>::visit_generics
  24:     0x7f607c47b4fb - <rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor>::process_generic_params
  25:     0x7f607c47f2b0 - <rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor as rustc_hir[7415ed71242d56dd]::intravisit::Visitor>::visit_item
  26:     0x7f607c484536 - <rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor as rustc_hir[7415ed71242d56dd]::intravisit::Visitor>::visit_nested_item
  27:     0x7f607c465817 - rustc_hir[7415ed71242d56dd]::intravisit::walk_mod::<rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor>
  28:     0x7f607c47cfad - <rustc_save_analysis[df44e3eec8415474]::dump_visitor::DumpVisitor>::process_crate
  29:     0x7f607c42e18c - <rustc_middle[f4e20aeb7daf39a5]::dep_graph::dep_node::DepKind as rustc_query_system[8761e2140374fac1]::dep_graph::DepKind>::with_deps::<rustc_save_analysis[df44e3eec8415474]::process_crate<rustc_save_analysis[df44e3eec8415474]::DumpHandler>::{closure#0}::{closure#0}, ()>
  30:     0x7f607c44f998 - <std[478b86191f7cb4e1]::thread::local::LocalKey<core[d8150d240427f197]::cell::Cell<bool>>>::with::<rustc_middle[f4e20aeb7daf39a5]::ty::print::pretty::with_no_trimmed_paths<rustc_save_analysis[df44e3eec8415474]::process_crate<rustc_save_analysis[df44e3eec8415474]::DumpHandler>::{closure#0}, ()>::{closure#0}, ()>
  31:     0x7f607c44fb7d - rustc_save_analysis[df44e3eec8415474]::process_crate::<rustc_save_analysis[df44e3eec8415474]::DumpHandler>
  32:     0x7f607c4242f5 - <rustc_session[575563c8b066c15a]::session::Session>::time::<(), rustc_driver[57ba6522360aaed]::run_compiler::{closure#1}::{closure#2}::{closure#3}::{closure#0}>
  33:     0x7f607e2d373b - <rustc_interface[d81b8f79c5ea87c1]::passes::QueryContext>::enter::<rustc_driver[57ba6522360aaed]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d8150d240427f197]::result::Result<(), rustc_errors[85f8821eedd20cf3]::ErrorReported>>
  34:     0x7f607e2c1ad2 - rustc_interface[d81b8f79c5ea87c1]::interface::create_compiler_and_run::<core[d8150d240427f197]::result::Result<(), rustc_errors[85f8821eedd20cf3]::ErrorReported>, rustc_driver[57ba6522360aaed]::run_compiler::{closure#1}>
  35:     0x7f607e2a71f2 - std[478b86191f7cb4e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d81b8f79c5ea87c1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[d81b8f79c5ea87c1]::interface::run_compiler<core[d8150d240427f197]::result::Result<(), rustc_errors[85f8821eedd20cf3]::ErrorReported>, rustc_driver[57ba6522360aaed]::run_compiler::{closure#1}>::{closure#0}, core[d8150d240427f197]::result::Result<(), rustc_errors[85f8821eedd20cf3]::ErrorReported>>::{closure#0}, core[d8150d240427f197]::result::Result<(), rustc_errors[85f8821eedd20cf3]::ErrorReported>>
  36:     0x7f607e2da309 - <<std[478b86191f7cb4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[d81b8f79c5ea87c1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[d81b8f79c5ea87c1]::interface::run_compiler<core[d8150d240427f197]::result::Result<(), rustc_errors[85f8821eedd20cf3]::ErrorReported>, rustc_driver[57ba6522360aaed]::run_compiler::{closure#1}>::{closure#0}, core[d8150d240427f197]::result::Result<(), rustc_errors[85f8821eedd20cf3]::ErrorReported>>::{closure#0}, core[d8150d240427f197]::result::Result<(), rustc_errors[85f8821eedd20cf3]::ErrorReported>>::{closure#1} as core[d8150d240427f197]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f607bcd5e13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hbafd1e8df522f967
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/alloc/src/boxed.rs:1854:9
  38:     0x7f607bcd5e13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h547d8dfcedefc79e
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/alloc/src/boxed.rs:1854:9
  39:     0x7f607bcd5e13 - std::sys::unix::thread::Thread::new::thread_start::h3722de690b04078c
                               at /rustc/5d8767cb229b097fedb1dd4bd9420d463c37774f/library/std/src/sys/unix/thread.rs:108:17
  40:     0x7f607bbdb259 - start_thread
  41:     0x7f607baf75e3 - __GI___clone
  42:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (5d8767cb2 2022-02-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z save-analysis

query stack during panic:
end of query stack
error: aborting due to previous error
