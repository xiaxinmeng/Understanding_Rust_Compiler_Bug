console
$ rustc --print=cfg > /dev/full
thread 'rustc' panicked at 'failed printing to stdout: No space left on device (os error 28)', library/std/src/io/stdio.rs:1019:9
stack backtrace:
   0:     0x7f12ddd68f33 - std::backtrace_rs::backtrace::libunwind::trace::h527d8d64d53ade2d
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f12ddd68f33 - std::backtrace_rs::backtrace::trace_unsynchronized::hfb55b01517dd6379
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f12ddd68f33 - std::sys_common::backtrace::_print_fmt::hd134e914eea0bd97
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f12ddd68f33 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1480db11ec399d77
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f12dddc9d4f - core::fmt::write::h67ec4c4171c92b26
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/core/src/fmt/mod.rs:1247:17
   5:     0x7f12ddd5bed1 - std::io::Write::write_fmt::h3b12aef0fff2463b
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/io/mod.rs:1712:15
   6:     0x7f12ddd68d45 - std::sys_common::backtrace::_print::h584400135abdbd51
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f12ddd68d45 - std::sys_common::backtrace::print::hce41d3c8bd91096b
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f12ddd6b84f - std::panicking::default_hook::{{closure}}::h2043b657a3791225
   9:     0x7f12ddd6b507 - std::panicking::default_hook::h99252b8d3dd5719c
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/panicking.rs:293:9
  10:     0x7f12e0fdf915 - <rustc_driver_impl[d30cd2737d9d343a]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[cc19a662f3570270]::ops::function::FnOnce<(&core[cc19a662f3570270]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f12ddd6c005 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h222a2b674b9f4762
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/alloc/src/boxed.rs:1976:9
  12:     0x7f12ddd6c005 - std::panicking::rust_panic_with_hook::h7f49b36bf7f8ff77
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/panicking.rs:704:13
  13:     0x7f12ddd6bd73 - std::panicking::begin_panic_handler::{{closure}}::haa23a7352589e31e
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/panicking.rs:595:13
  14:     0x7f12ddd69376 - std::sys_common::backtrace::__rust_end_short_backtrace::h3d0cf6e3c96e3fe9
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f12ddd6ba72 - rust_begin_unwind
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/panicking.rs:584:5
  16:     0x7f12dddc5fe3 - core::panicking::panic_fmt::hf4b4ea11e3fdb110
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/core/src/panicking.rs:67:14
  17:     0x7f12ddd5aa07 - std::io::stdio::print_to::h55760b9ede306280
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/io/stdio.rs:1019:9
  18:     0x7f12ddd5aa07 - std::io::stdio::_print::h93deb6099db33eab
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/io/stdio.rs:1095:5
  19:     0x7f12e01e163b - rustc_driver_impl[d30cd2737d9d343a]::print_crate_info
  20:     0x7f12e0fb8e01 - rustc_span[9551eaa044f53f4f]::set_source_map::<(), rustc_interface[c2b70c9b1dae0906]::interface::run_compiler<(), rustc_driver_impl[d30cd2737d9d343a]::run_compiler::{closure#0}>::{closure#0}::{closure#0}>
  21:     0x7f12e0fb6d59 - std[71a32ca0600a6a04]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c2b70c9b1dae0906]::util::run_in_thread_pool_with_globals<rustc_interface[c2b70c9b1dae0906]::interface::run_compiler<(), rustc_driver_impl[d30cd2737d9d343a]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>
  22:     0x7f12e0fe25b4 - std[71a32ca0600a6a04]::panicking::try::<(), core[cc19a662f3570270]::panic::unwind_safe::AssertUnwindSafe<<std[71a32ca0600a6a04]::thread::Builder>::spawn_unchecked_<rustc_interface[c2b70c9b1dae0906]::util::run_in_thread_pool_with_globals<rustc_interface[c2b70c9b1dae0906]::interface::run_compiler<(), rustc_driver_impl[d30cd2737d9d343a]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#1}::{closure#0}>>
  23:     0x7f12e0fb0cf1 - <<std[71a32ca0600a6a04]::thread::Builder>::spawn_unchecked_<rustc_interface[c2b70c9b1dae0906]::util::run_in_thread_pool_with_globals<rustc_interface[c2b70c9b1dae0906]::interface::run_compiler<(), rustc_driver_impl[d30cd2737d9d343a]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#1} as core[cc19a662f3570270]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f12ddd763e5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf15d802f31f86225
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/alloc/src/boxed.rs:1962:9
  25:     0x7f12ddd763e5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hac564355b46c52d6
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/alloc/src/boxed.rs:1962:9
  26:     0x7f12ddd763e5 - std::sys::unix::thread::Thread::new::thread_start::h86fb3aedb7811f07
                               at /rustc/7f94b314cead7059a71a265a8b64905ef2511796/library/std/src/sys/unix/thread.rs:108:17
  27:     0x7f12dda94b43 - start_thread
                               at ./nptl/pthread_create.c:442:8
  28:     0x7f12ddb26a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  29:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (7f94b314c 2023-04-23) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
