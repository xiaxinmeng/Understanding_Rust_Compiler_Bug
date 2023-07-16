
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `false`,
 right: `true`', a.rs:10:5
stack backtrace:
   0:     0x557cb620a58a - std::backtrace_rs::backtrace::libunwind::trace::ha9053a9a07ca49cb
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x557cb620a58a - std::backtrace_rs::backtrace::trace_unsynchronized::h9c2852a457ad564e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x557cb620a58a - std::sys_common::backtrace::_print_fmt::h457936fbfaa0070f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x557cb620a58a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5779d7bf7f70cb0c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x557cb6223d9e - core::fmt::write::h5a4baaff1bcd3eb5
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/fmt/mod.rs:1232:17
   5:     0x557cb6208715 - std::io::Write::write_fmt::h4bc1f301cb9e9cce
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/io/mod.rs:1684:15
   6:     0x557cb620a355 - std::sys_common::backtrace::_print::h5fcdc36060f177e8
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x557cb620a355 - std::sys_common::backtrace::print::h54ca9458b876c8bf
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x557cb620b76f - std::panicking::default_hook::{{closure}}::hbe471161c7664ed6
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:271:22
   9:     0x557cb620b4ab - std::panicking::default_hook::ha3500da57aa4ac4f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:290:9
  10:     0x557cb620bd18 - std::panicking::rust_panic_with_hook::h50c09d000dc561d2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:692:13
  11:     0x557cb620bc19 - std::panicking::begin_panic_handler::{{closure}}::h9e2b2176e00e0d9c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:583:13
  12:     0x557cb620a9f6 - std::sys_common::backtrace::__rust_end_short_backtrace::h5739b8e512c09d02
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:150:18
  13:     0x557cb620b922 - rust_begin_unwind
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:579:5
  14:     0x557cb61f2ed3 - core::panicking::panic_fmt::hf33a1475b4dc5c3e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:64:14
  15:     0x557cb61f314f - core::panicking::assert_failed_inner::haf9816227b20b6f2
  16:     0x557cb61f36c6 - core::panicking::assert_failed::hfe4604f1e2156329
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:211:5
