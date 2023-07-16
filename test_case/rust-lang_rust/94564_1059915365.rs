
(lldb) bt
* thread #1, name = 'ltocrash', stop reason = signal SIGABRT
  * frame #0: 0x00007ffff7f1682b ltocrash`gsignal + 203
    frame #1: 0x00007ffff7ecbe1e ltocrash`abort + 299
    frame #2: 0x00007ffff7edf127 ltocrash`std::sys::unix::abort_internal::hcaf26b2b5da2de51 at mod.rs:259:14
    frame #3: 0x00007ffff7f052ae ltocrash`std::panicking::rust_panic_with_hook::hfa6a9afb1a6b2eff at panicking.rs:682:9
    frame #4: 0x00007ffff7f04ebb ltocrash`std::panicking::begin_panic_handler::_$u7b$$u7b$closure$u7d$$u7d$::h6b37cfd8e0c1dd8a at panicking.rs:586:13
    frame #5: 0x00007ffff7f04e56 ltocrash`std::sys_common::backtrace::__rust_end_short_backtrace::hc59fb3f99f6cb43f at backtrace.rs:138:18
    frame #6: 0x00007ffff7f04e12 ltocrash`rust_begin_unwind at panicking.rs:584:5
    frame #7: 0x00007ffff7eca392 ltocrash`core::panicking::panic_fmt::h7275fb82410a6b0a at panicking.rs:143:14
    frame #8: 0x00007ffff7f05301 ltocrash`std::panicking::rust_panic_with_hook::hfa6a9afb1a6b2eff [inlined] std::sys::unix::rwlock::RWLock::read::hb91739d041bece46 at rwlock.rs:0
    frame #9: 0x00007ffff7f052b0 ltocrash`std::panicking::rust_panic_with_hook::hfa6a9afb1a6b2eff [inlined] std::sys_common::rwlock::StaticRWLock::read::hb0a5647500e474ee at rwlock.rs:23
    frame #10: 0x00007ffff7f052b0 ltocrash`std::panicking::rust_panic_with_hook::hfa6a9afb1a6b2eff at panicking.rs:687
    frame #11: 0x00007ffff7f04ebb ltocrash`std::panicking::begin_panic_handler::_$u7b$$u7b$closure$u7d$$u7d$::h6b37cfd8e0c1dd8a at panicking.rs:586:13
    frame #12: 0x00007ffff7f04e56 ltocrash`std::sys_common::backtrace::__rust_end_short_backtrace::hc59fb3f99f6cb43f at backtrace.rs:138:18
    frame #13: 0x00007ffff7f04e12 ltocrash`rust_begin_unwind at panicking.rs:584:5
    frame #14: 0x00007ffff7eca392 ltocrash`core::panicking::panic_fmt::h7275fb82410a6b0a at panicking.rs:143:14
    frame #15: 0x00007ffff7f05301 ltocrash`std::panicking::rust_panic_with_hook::hfa6a9afb1a6b2eff [inlined] std::sys::unix::rwlock::RWLock::read::hb91739d041bece46 at rwlock.rs:0
    frame #16: 0x00007ffff7f052b0 ltocrash`std::panicking::rust_panic_with_hook::hfa6a9afb1a6b2eff [inlined] std::sys_common::rwlock::StaticRWLock::read::hb0a5647500e474ee at rwlock.rs:23
    frame #17: 0x00007ffff7f052b0 ltocrash`std::panicking::rust_panic_with_hook::hfa6a9afb1a6b2eff at panicking.rs:687
    frame #18: 0x00007ffff7f04ebb ltocrash`std::panicking::begin_panic_handler::_$u7b$$u7b$closure$u7d$$u7d$::h6b37cfd8e0c1dd8a at panicking.rs:586:13
    frame #19: 0x00007ffff7f04e56 ltocrash`std::sys_common::backtrace::__rust_end_short_backtrace::hc59fb3f99f6cb43f at backtrace.rs:138:18
    frame #20: 0x00007ffff7f04e12 ltocrash`rust_begin_unwind at panicking.rs:584:5
    frame #21: 0x00007ffff7eca392 ltocrash`core::panicking::panic_fmt::h7275fb82410a6b0a at panicking.rs:143:14
    frame #22: 0x00007ffff7f0258b ltocrash`std::process::Command::spawn::h9eaebc0e504e67be [inlined] std::sys::unix::rwlock::RWLock::read::hb91739d041bece46 at rwlock.rs:49:13
    frame #23: 0x00007ffff7f02528 ltocrash`std::process::Command::spawn::h9eaebc0e504e67be [inlined] std::sys_common::rwlock::StaticRWLock::read::hb0a5647500e474ee at rwlock.rs:23
    frame #24: 0x00007ffff7f02528 ltocrash`std::process::Command::spawn::h9eaebc0e504e67be [inlined] std::sys::unix::os::env_read_lock::hb7ce90cb70e0dd99 at os.rs:487
    frame #25: 0x00007ffff7f02528 ltocrash`std::process::Command::spawn::h9eaebc0e504e67be [inlined] std::sys::unix::process::process_inner::_$LT$impl$u20$std..sys..unix..process..process_common..Command$GT$::posix_spawn::hf188deaf768f5228 at process_unix.rs:526
    frame #26: 0x00007ffff7f02528 ltocrash`std::process::Command::spawn::h9eaebc0e504e67be [inlined] std::sys::unix::process::process_inner::_$LT$impl$u20$std..sys..unix..process..process_common..Command$GT$::spawn::h27d6f54ab4e33566 at process_unix.rs:55
    frame #27: 0x00007ffff7f00af2 ltocrash`std::process::Command::spawn::h9eaebc0e504e67be at process.rs:868
    frame #28: 0x00007ffff7ecd03c ltocrash`ltocrash::main::h1695d9afc4b3cdf2 + 604
    frame #29: 0x00007ffff7ed4543 ltocrash`std::sys_common::backtrace::__rust_begin_short_backtrace::hf29c11af97155265 + 3
    frame #30: 0x00007ffff7ecd730 ltocrash`main + 1296
    frame #31: 0x00007ffff7f095b0 ltocrash`__libc_start_main + 1168
    frame #32: 0x00007ffff7ecc72e ltocrash`_start + 46
