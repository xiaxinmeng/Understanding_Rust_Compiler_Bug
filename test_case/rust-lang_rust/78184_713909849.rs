
$ rustup default 1.46.0
info: using existing install for '1.46.0-x86_64-unknown-freebsd'
info: default toolchain set to '1.46.0-x86_64-unknown-freebsd'

  1.46.0-x86_64-unknown-freebsd unchanged - rustc 1.46.0 (04488afe3 2020-08-24)

$ rustc --version
rustc 1.46.0 (04488afe3 2020-08-24)
$ RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/foo`
thread 'main' panicked at 'Hello, world!', src/main.rs:2:5
stack backtrace:
   0:          0x1036a5d - backtrace::backtrace::libunwind::trace::h55d34efd667ae7f2
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:          0x1036a5d - backtrace::backtrace::trace_unsynchronized::h22e345d4ced2d4ad
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:          0x1036a5d - std::sys_common::backtrace::_print_fmt::h19199d69d36cf141
                               at src/libstd/sys_common/backtrace.rs:78
   3:          0x1036a5d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf4a5883ac429a8c0
                               at src/libstd/sys_common/backtrace.rs:59
   4:          0x105325c - core::fmt::write::h034d3b092e0f8bf8
                               at src/libcore/fmt/mod.rs:1076
   5:          0x1035082 - std::io::Write::write_fmt::hcd59a10b4c372132
                               at src/libstd/io/mod.rs:1537
   6:          0x1038808 - std::sys_common::backtrace::_print::h697b7cdcde768e1b
                               at src/libstd/sys_common/backtrace.rs:62
   7:          0x1038808 - std::sys_common::backtrace::print::h2cbe97e71b9f43bf
                               at src/libstd/sys_common/backtrace.rs:49
   8:          0x1038808 - std::panicking::default_hook::{{closure}}::hd560d2eff188e8e5
                               at src/libstd/panicking.rs:198
   9:          0x10384c9 - std::panicking::default_hook::h3225be6477d52e2e
                               at src/libstd/panicking.rs:217
  10:          0x1038e65 - std::panicking::rust_panic_with_hook::h10d7ad778c2ddef3
                               at src/libstd/panicking.rs:526
  11:          0x1030986 - std::panicking::begin_panic::h91786f982b531f94
                               at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/libstd/panicking.rs:456
  12:          0x10306ec - foo::main::h967e1f2bf0ad1d46
                               at src/main.rs:2
  13:          0x103084e - std::rt::lang_start::{{closure}}::hcd91829aefc439cf
                               at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/libstd/rt.rs:67
  14:          0x10391ea - std::rt::lang_start_internal::{{closure}}::hc0f35a74710e4a19
                               at src/libstd/rt.rs:52
  15:          0x10391ea - std::panicking::try::do_call::hf3de4c4c1dce977f
                               at src/libstd/panicking.rs:348
  16:          0x10391ea - std::panicking::try::h8b2e697ba94a3253
                               at src/libstd/panicking.rs:325
  17:          0x10391ea - std::panic::catch_unwind::hc4dd4672f0cc4abb
                               at src/libstd/panic.rs:394
  18:          0x10391ea - std::rt::lang_start_internal::ha90c4c8e282f65c1
                               at src/libstd/rt.rs:51
  19:          0x1030832 - std::rt::lang_start::hc1f9ef8afcf5202f
                               at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/libstd/rt.rs:67
  20:          0x103071b - main
  21:          0x103010b - _start
                               at /usr/src/lib/csu/amd64/crt1.c:76
