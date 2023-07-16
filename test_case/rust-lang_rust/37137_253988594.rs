
thread 'raftstore-4' panicked 'other was less than the current instant' at "../src/libstd/sys/unix/time.rs:276"
stack backtrace:
   0:     0x7fdb0fd53c1e - backtrace::backtrace::libunwind::trace
                        at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.2.3/src/backtrace/libunwind.rs:54
                         - backtrace::backtrace::trace<closure>
                        at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.2.3/src/backtrace/mod.rs:70
   1:     0x7fdb0fd54333 - backtrace::capture::{{impl}}::new
                        at /tikv/target/release/build/backtrace-3bc1ff360ebb00cb/out/capture.rs:79
   2:     0x7fdb0fd53540 - tikv::util::panic_hook::set_exit_hook::{{closure}}
                        at /tikv/src/util/panic_hook.rs:84
   3:     0x7fdb102863f7 - std::panicking::rust_panic_with_hook::h105c3d42fcd2fb5e
   4:     0x7fdb1028628f - std::panicking::begin_panic::h5c54e97e5cda1cc4
   5:     0x7fdb1027e5e9 - std::time::Instant::elapsed::h032d2251de7f06d6
