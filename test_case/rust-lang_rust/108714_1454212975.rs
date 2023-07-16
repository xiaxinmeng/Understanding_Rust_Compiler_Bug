
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', /home/gh-estebank/.cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.1/src/snapshot_vec.rs:199:10
stack backtrace:
   0:     0x7f258adc1627 - std::backtrace_rs::backtrace::libunwind::trace::hfb636e2537229b31
                               at /home/gh-estebank/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f258adc1627 - std::backtrace_rs::backtrace::trace_unsynchronized::h66950c0f6b5094ed
                               at /home/gh-estebank/rust/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f258adc5b97 - std::sys_common::backtrace::_print_fmt::h52017e4d45ad3a1f
                               at /home/gh-estebank/rust/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f258adc5b97 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h79241e87a3cbcb2c
                               at /home/gh-estebank/rust/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f258ae0effc - core::fmt::write::he52e24bcdfa17bcc
                               at /home/gh-estebank/rust/library/core/src/fmt/mod.rs:1232:17
   5:     0x7f258ad83ff1 - std::io::Write::write_fmt::hfcf2df9a236b625e
                               at /home/gh-estebank/rust/library/std/src/io/mod.rs:1684:15
   6:     0x7f258adc59fb - std::sys_common::backtrace::_print::h342e967f1eb93a26
                               at /home/gh-estebank/rust/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f258adc59fb - std::sys_common::backtrace::print::h0e4a000db936879b
                               at /home/gh-estebank/rust/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f258ad71600 - std::panicking::panic_hook_with_disk_dump::{{closure}}::hf11dd0b5bbad5835
   9:     0x7f258ad712ef - std::panicking::panic_hook_with_disk_dump::hff6fb45faca45d68
                               at /home/gh-estebank/rust/library/std/src/panicking.rs:307:9
  10:     0x7f258b7f2cf8 - rustc_driver_impl[81fde20db5359143]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/gh-estebank/rust/compiler/rustc_driver_impl/src/lib.rs:1203:17
8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8<
 309:     0x7f258ad8eea8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0103095a1dd1d58e
                               at /home/gh-estebank/rust/library/alloc/src/boxed.rs:1987:9
 310:     0x7f258ad8eea8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h5b4c3dea47ef3a49
                               at /home/gh-estebank/rust/library/alloc/src/boxed.rs:1987:9
 311:     0x7f258ad7bb7a - std::sys::unix::thread::Thread::new::thread_start::hf0c056e426350bd2
                               at /home/gh-estebank/rust/library/std/src/sys/unix/thread.rs:108:17
 312:     0x7f258ab51b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
 313:     0x7f258abe3a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
 314:                0x0 - <unknown>


rustc version: 1.69.0-dev
platform: x86_64-unknown-linux-gnu

query stack during panic:
#0 [codegen_select_candidate] computing candidate for `<core::result::Result<(), _> as core::ops::try_trait::Try>`
#1 [resolve_instance] resolving instance `<core::result::Result<(), _> as core::ops::try_trait::Try>::branch`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
