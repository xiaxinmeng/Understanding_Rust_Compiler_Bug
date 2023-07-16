
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `false`,
 right: `true`', a.rs:10:5
stack backtrace:
   0:     0x555d9bd4c5aa - std::backtrace_rs::backtrace::libunwind::trace::ha9053a9a07ca49cb
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x555d9bd4c5aa - std::backtrace_rs::backtrace::trace_unsynchronized::h9c2852a457ad564e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x555d9bd4c5aa - std::sys_common::backtrace::_print_fmt::h457936fbfaa0070f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x555d9bd4c5aa - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5779d7bf7f70cb0c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x555d9bd6620e - core::fmt::write::h5a4baaff1bcd3eb5
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/fmt/mod.rs:1232:17
   5:     0x555d9bd4a735 - std::io::Write::write_fmt::h4bc1f301cb9e9cce
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/io/mod.rs:1684:15
   6:     0x555d9bd4c375 - std::sys_common::backtrace::_print::h5fcdc36060f177e8
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x555d9bd4c375 - std::sys_common::backtrace::print::h54ca9458b876c8bf
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x555d9bd4d9cf - std::panicking::default_hook::{{closure}}::hbe471161c7664ed6
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:271:22
   9:     0x555d9bd4d70b - std::panicking::default_hook::ha3500da57aa4ac4f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:290:9
  10:     0x555d9bd4df78 - std::panicking::rust_panic_with_hook::h50c09d000dc561d2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:692:13
  11:     0x555d9bd4de79 - std::panicking::begin_panic_handler::{{closure}}::h9e2b2176e00e0d9c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:583:13
  12:     0x555d9bd4ca16 - std::sys_common::backtrace::__rust_end_short_backtrace::h5739b8e512c09d02
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:150:18
  13:     0x555d9bd4db82 - rust_begin_unwind
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:579:5
  14:     0x555d9bd34ed3 - core::panicking::panic_fmt::hf33a1475b4dc5c3e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:64:14
  15:     0x555d9bd3514f - core::panicking::assert_failed_inner::haf9816227b20b6f2
  16:     0x555d9bd356e6 - core::panicking::assert_failed::hfe4604f1e2156329
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:211:5
  17:     0x555d9bd355d2 - a::bar::h54068ed5e62eaea6
                               at /tmp/a/a.rs:10:5
  18:     0x555d9bd35586 - a::foo::hbe93c09168c5d787
                               at /tmp/a/a.rs:6:5
  19:     0x555d9bd35576 - a::main::hf778f5e35fefb56f
                               at /tmp/a/a.rs:2:5
  20:     0x555d9bd35503 - core::ops::function::FnOnce::call_once::h0cda876b3b6f352f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
  21:     0x555d9bd354e9 - std::sys_common::backtrace::__rust_begin_short_backtrace::hcd8ad9bf1b410bdc
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:134:18
  22:     0x555d9bd35659 - std::rt::lang_start::{{closure}}::h135e76f90c71d025
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:166:18
  23:     0x555d9bd48a6c - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hd6efcd3bec896f2c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:287:13
  24:     0x555d9bd48a6c - std::panicking::try::do_call::hce04e543bb1f4cbb
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  25:     0x555d9bd48a6c - std::panicking::try::h3342dd4e1f680968
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  26:     0x555d9bd48a6c - std::panic::catch_unwind::h148ce1e59ac0cee7
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  27:     0x555d9bd48a6c - std::rt::lang_start_internal::{{closure}}::h25f9dda2057a67fe
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:148:48
  28:     0x555d9bd48a6c - std::panicking::try::do_call::h7caaaeaf9401650b
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  29:     0x555d9bd48a6c - std::panicking::try::he7d15285746cbbc2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  30:     0x555d9bd48a6c - std::panic::catch_unwind::h89fb4f50c0301fe0
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  31:     0x555d9bd48a6c - std::rt::lang_start_internal::h078acd489417d3c1
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:148:20
  32:     0x555d9bd35637 - std::rt::lang_start::h123fe1c2f5ee3bc1
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:165:17
  33:     0x555d9bd355fe - main
  34:     0x7f78e854c850 - <unknown>
  35:     0x7f78e854c90a - __libc_start_main
  36:     0x555d9bd35405 - _start
  37:                0x0 - <unknown>
