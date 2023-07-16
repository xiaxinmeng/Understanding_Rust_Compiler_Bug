
thread '<unnamed>' panicked at '!!!', src/lib.rs:6:9
stack backtrace:
   0:     0x7efbff47ad04 - backtrace::backtrace::libunwind::trace::h65597d255cb1398b
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7efbff47ad04 - backtrace::backtrace::trace_unsynchronized::hd4f479d7150ec4a0
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7efbff47ad04 - std::sys_common::backtrace::_print_fmt::h015072984a2b172c
                               at src/libstd/sys_common/backtrace.rs:77
   3:     0x7efbff47ad04 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6df05d3335f32194
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7efbff49265c - core::fmt::write::h1f444f4312eb6c27
                               at src/libcore/fmt/mod.rs:1028
   5:     0x7efbff479c37 - std::io::Write::write_fmt::h8d147888220078ef
                               at src/libstd/io/mod.rs:1412
   6:     0x7efbff47cc2e - std::sys_common::backtrace::_print::h8a6df0fa81d6af62
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7efbff47cc2e - std::sys_common::backtrace::print::h6f05b4733407e509
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7efbff47cc2e - std::panicking::default_hook::{{closure}}::h0d0a23bd02315dd8
                               at src/libstd/panicking.rs:188
   9:     0x7efbff47c921 - std::panicking::default_hook::h8d15a9aecb4efac6
                               at src/libstd/panicking.rs:205
  10:     0x7efbff47d25b - std::panicking::rust_panic_with_hook::hbe174577402a475d
                               at src/libstd/panicking.rs:464
  11:     0x7efbff47634f - std::panicking::begin_panic::h634f0a45cd79b13c
                               at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libstd/panicking.rs:400
  12:     0x7efbff47760d - reloadable::RELOAD_API::{{closure}}::h6be6a6421c7d5bd6
                               at src/lib.rs:6
  13:     0x7efbff476b3e - core::ops::function::FnOnce::call_once::hd3d414ac216f7262
                               at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/ops/function.rs:227
  14:     0x565555559cb5 - libreloading_test::main::h100c07e459594b27
                               at src/main.rs:16
...
