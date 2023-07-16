
 $ rustc --version
rustc 1.49.0-nightly (688c68b99 2020-10-26)

$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/foo`
thread 'main' panicked at 'Hello, world!', src/main.rs:2:5
stack backtrace:
   0: std::panicking::begin_panic
             at /rustc/688c68b99d9ece86c9b2389ccaff13c442bcae2e/library/std/src/panicking.rs:521:12
   1: foo::main
             at ./src/main.rs:2:5
   2: core::ops::function::FnOnce::call_once
             at /rustc/688c68b99d9ece86c9b2389ccaff13c442bcae2e/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

$ RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/foo`
thread 'main' panicked at 'Hello, world!', src/main.rs:2:5
stack backtrace:
   0:          0x1046341 - std::backtrace_rs::backtrace::libunwind::trace::h38697ba0962b0839
                               at /rustc/688c68b99d9ece86c9b2389ccaff13c442bcae2e/library/std/src/../../backtrace/src/backtrace/libunwind.rs:100:5
   1:          0x1046341 - std::backtrace_rs::backtrace::trace_unsynchronized::h5ef4749c21f60963
                               at /rustc/688c68b99d9ece86c9b2389ccaff13c442bcae2e/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:          0x1046341 - std::sys_common::backtrace::_print_fmt::h49193578845187a0
                               at /rustc/688c68b99d9ece86c9b2389ccaff13c442bcae2e/library/std/src/sys_common/backtrace.rs:67:5
   3:          0x1046341 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb7f4416a63f42308
                               at /rustc/688c68b99d9ece86c9b2389ccaff13c442bcae2e/library/std/src/sys_common/backtrace.rs:46:22
   4:          0x105e2b0 - core::fmt::write::heec8bbbd722d7369
                               at /rustc/688c68b99d9ece86c9b2389ccaff13c442bcae2e/library/core/src/fmt/mod.rs:1076:17
...
