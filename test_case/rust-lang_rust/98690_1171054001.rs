
$ rustdoc --test -Z unstable-options --persist-doctests /tmp/foobar main.rs 
thread 'rustc' panicked at 'Couldn't create directory for doctest executables: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }', src/librustdoc/doctest.rs:1007:18
stack backtrace:
   0:     0x7faa373d65cd - std::backtrace_rs::backtrace::libunwind::trace::h79327ccd22e2164a
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7faa373d65cd - std::backtrace_rs::backtrace::trace_unsynchronized::h7474da7e45f82fef
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7faa373d65cd - std::sys_common::backtrace::_print_fmt::h6c7edbb20ad2f451
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7faa373d65cd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed6df72de1722661
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7faa3742f90c - core::fmt::write::h11f5d0f6418e15ab
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/core/src/fmt/mod.rs:1198:17
   5:     0x7faa373c7c41 - std::io::Write::write_fmt::h2287344a97990f36
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/io/mod.rs:1672:15
   6:     0x7faa373d9295 - std::sys_common::backtrace::_print::h5503bb3e2c3786ac
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7faa373d9295 - std::sys_common::backtrace::print::hd93eff6347adc6af
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7faa373d9295 - std::panicking::default_hook::{{closure}}::h77c8a625f7da619f
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/panicking.rs:295:22
   9:     0x7faa373d8fb6 - std::panicking::default_hook::h975ef4f484179dc7
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/panicking.rs:314:9
  10:     0x7faa37bd43f4 - rustc_driver[bb5018aaf4248976]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7faa373d996a - std::panicking::rust_panic_with_hook::h4c5237b27a09590d
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/panicking.rs:702:17
  12:     0x7faa373d97a7 - std::panicking::begin_panic_handler::{{closure}}::h083e2d2917996848
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/panicking.rs:588:13
  13:     0x7faa373d6a84 - std::sys_common::backtrace::__rust_end_short_backtrace::h90649445b345543f
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7faa373d94d9 - rust_begin_unwind
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/panicking.rs:584:5
  15:     0x7faa3739e7e3 - core::panicking::panic_fmt::h6f9d2b4a0d7a26a5
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/core/src/panicking.rs:142:14
  16:     0x7faa3739e933 - core::result::unwrap_failed::h591433f408423ad6
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/core/src/result.rs:1805:5
  17:     0x559b59a87be9 - <rustdoc[831424ccce498430]::doctest::Collector as rustdoc[831424ccce498430]::doctest::Tester>::add_test
  18:     0x559b59ac9d51 - rustdoc[831424ccce498430]::html::markdown::find_testable_code::<rustdoc[831424ccce498430]::doctest::Collector>
  19:     0x559b59a88e71 - <rustdoc[831424ccce498430]::doctest::HirCollector as rustc_hir[67106ace806a6a70]::intravisit::Visitor>::visit_item
  20:     0x559b59a8878e - <rustdoc[831424ccce498430]::doctest::HirCollector>::visit_testable::<rustdoc[831424ccce498430]::doctest::run::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  21:     0x559b59ae5829 - <rustc_interface[8131206a6d8b1989]::interface::Compiler>::enter::<rustdoc[831424ccce498430]::doctest::run::{closure#1}::{closure#0}, core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>>
  22:     0x559b598e8130 - rustc_span[25b14a9e1900d955]::with_source_map::<core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>, rustc_interface[8131206a6d8b1989]::interface::create_compiler_and_run<core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>, rustdoc[831424ccce498430]::doctest::run::{closure#1}>::{closure#1}>
  23:     0x559b59b15a4f - rustc_interface[8131206a6d8b1989]::interface::create_compiler_and_run::<core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>, rustdoc[831424ccce498430]::doctest::run::{closure#1}>
  24:     0x559b598e9920 - <scoped_tls[8c665c213b0e261f]::ScopedKey<rustc_span[25b14a9e1900d955]::SessionGlobals>>::set::<rustc_interface[8131206a6d8b1989]::interface::run_compiler<core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>, rustdoc[831424ccce498430]::doctest::run::{closure#1}>::{closure#0}, core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>>
  25:     0x559b59a5fa5c - std[da9e0d8df6b8e369]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8131206a6d8b1989]::util::run_in_thread_pool_with_globals<rustc_interface[8131206a6d8b1989]::interface::run_compiler<core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>, rustdoc[831424ccce498430]::doctest::run::{closure#1}>::{closure#0}, core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>>::{closure#0}, core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>>
  26:     0x559b59b86927 - <<std[da9e0d8df6b8e369]::thread::Builder>::spawn_unchecked_<rustc_interface[8131206a6d8b1989]::util::run_in_thread_pool_with_globals<rustc_interface[8131206a6d8b1989]::interface::run_compiler<core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>, rustdoc[831424ccce498430]::doctest::run::{closure#1}>::{closure#0}, core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>>::{closure#0}, core[50a63c015f38975a]::result::Result<(alloc[cd27a1bb7ed2ecb3]::vec::Vec<test[d639fab4c9ac2c08]::types::TestDescAndFn>, alloc[cd27a1bb7ed2ecb3]::sync::Arc<std[da9e0d8df6b8e369]::sync::mutex::Mutex<alloc[cd27a1bb7ed2ecb3]::vec::Vec<rustdoc[831424ccce498430]::doctest::UnusedExterns>>>, usize), rustc_errors[ace5696c6a45b28d]::ErrorGuaranteed>>::{closure#1} as core[50a63c015f38975a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7faa373e34e3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h22d0ff7b39ccc53a
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/alloc/src/boxed.rs:1951:9
  28:     0x7faa373e34e3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hef50bc84dfaa11d1
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/alloc/src/boxed.rs:1951:9
  29:     0x7faa373e34e3 - std::sys::unix::thread::Thread::new::thread_start::h07be67d12c7b8c55
                               at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/sys/unix/thread.rs:108:17
  30:     0x7faa372cb609 - start_thread
                               at /build/glibc-SzIz7B/glibc-2.31/nptl/pthread_create.c:477:8
  31:     0x7faa370a1133 - clone
                               at /build/glibc-SzIz7B/glibc-2.31/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  32:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (ddcbba036 2022-06-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options

query stack during panic:
end of query stack
