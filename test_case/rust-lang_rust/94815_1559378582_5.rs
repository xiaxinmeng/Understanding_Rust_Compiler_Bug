
thread 'main' panicked at 'expected false == true but they weren't', a.rs:10:5
stack backtrace:
   0:     0x564f3069467a - std::backtrace_rs::backtrace::libunwind::trace::ha9053a9a07ca49cb
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x564f3069467a - std::backtrace_rs::backtrace::trace_unsynchronized::h9c2852a457ad564e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x564f3069467a - std::sys_common::backtrace::_print_fmt::h457936fbfaa0070f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x564f3069467a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5779d7bf7f70cb0c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x564f306ae2de - core::fmt::write::h5a4baaff1bcd3eb5
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/fmt/mod.rs:1232:17
   5:     0x564f30692805 - std::io::Write::write_fmt::h4bc1f301cb9e9cce
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/io/mod.rs:1684:15
   6:     0x564f30694445 - std::sys_common::backtrace::_print::h5fcdc36060f177e8
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x564f30694445 - std::sys_common::backtrace::print::h54ca9458b876c8bf
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x564f30695a9f - std::panicking::default_hook::{{closure}}::hbe471161c7664ed6
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:271:22
   9:     0x564f306957db - std::panicking::default_hook::ha3500da57aa4ac4f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:290:9
  10:     0x564f30696048 - std::panicking::rust_panic_with_hook::h50c09d000dc561d2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:692:13
  11:     0x564f30695f49 - std::panicking::begin_panic_handler::{{closure}}::h9e2b2176e00e0d9c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:583:13
  12:     0x564f30694ae6 - std::sys_common::backtrace::__rust_end_short_backtrace::h5739b8e512c09d02
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:150:18
  13:     0x564f30695c52 - rust_begin_unwind
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:579:5
  14:     0x564f3067ced3 - core::panicking::panic_fmt::hf33a1475b4dc5c3e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:64:14
  15:     0x564f3067d5fd - a::bar::h54068ed5e62eaea6
                               at /tmp/a/a.rs:10:5
  16:     0x564f3067d576 - a::foo::hbe93c09168c5d787
                               at /tmp/a/a.rs:6:5
  17:     0x564f3067d566 - a::main::hf778f5e35fefb56f
                               at /tmp/a/a.rs:2:5
  18:     0x564f3067d503 - core::ops::function::FnOnce::call_once::h0cda876b3b6f352f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
  19:     0x564f3067d4e9 - std::sys_common::backtrace::__rust_begin_short_backtrace::hcd8ad9bf1b410bdc
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:134:18
  20:     0x564f3067d679 - std::rt::lang_start::{{closure}}::h135e76f90c71d025
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:166:18
  21:     0x564f30690b3c - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hd6efcd3bec896f2c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:287:13
  22:     0x564f30690b3c - std::panicking::try::do_call::hce04e543bb1f4cbb
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  23:     0x564f30690b3c - std::panicking::try::h3342dd4e1f680968
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  24:     0x564f30690b3c - std::panic::catch_unwind::h148ce1e59ac0cee7
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  25:     0x564f30690b3c - std::rt::lang_start_internal::{{closure}}::h25f9dda2057a67fe
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:148:48
  26:     0x564f30690b3c - std::panicking::try::do_call::h7caaaeaf9401650b
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  27:     0x564f30690b3c - std::panicking::try::he7d15285746cbbc2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  28:     0x564f30690b3c - std::panic::catch_unwind::h89fb4f50c0301fe0
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  29:     0x564f30690b3c - std::rt::lang_start_internal::h078acd489417d3c1
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:148:20
  30:     0x564f3067d657 - std::rt::lang_start::h123fe1c2f5ee3bc1
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:165:17
  31:     0x564f3067d61e - main
  32:     0x7fbbad08d850 - <unknown>
  33:     0x7fbbad08d90a - __libc_start_main
  34:     0x564f3067d405 - _start
  35:                0x0 - <unknown>
