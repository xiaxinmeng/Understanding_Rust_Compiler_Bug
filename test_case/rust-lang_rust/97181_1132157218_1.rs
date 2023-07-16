
thread 'main' panicked at 'not yet implemented', src/main.rs:7:9
stack backtrace:
   0:     0x563571da3b8d - std::backtrace_rs::backtrace::libunwind::trace::h22893a5306c091b4
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x563571da3b8d - std::backtrace_rs::backtrace::trace_unsynchronized::h29c3bc6f9e91819d
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x563571da3b8d - std::sys_common::backtrace::_print_fmt::he497d8a0ec903793
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x563571da3b8d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9c2a9d2774d81873
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x563571dbd85c - core::fmt::write::hba4337c43d992f49
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/fmt/mod.rs:1194:17
   5:     0x563571da1ee1 - std::io::Write::write_fmt::heb73de6e02cfabed
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/io/mod.rs:1655:15
   6:     0x563571da5335 - std::sys_common::backtrace::_print::h63c8b24acdd8e8ce
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x563571da5335 - std::sys_common::backtrace::print::h426700d6240cdcc2
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x563571da5335 - std::panicking::default_hook::{{closure}}::hc9a76eed0b18f82b
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:295:22
   9:     0x563571da4fe9 - std::panicking::default_hook::h2e88d02087fae196
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:314:9
  10:     0x563571da5882 - std::panicking::rust_panic_with_hook::habfdcc2e90f9fd4c
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:698:17
  11:     0x563571da5729 - std::panicking::begin_panic_handler::{{closure}}::he054b2a83a51d2cd
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:586:13
  12:     0x563571da4044 - std::sys_common::backtrace::__rust_end_short_backtrace::ha48b94ab49b30915
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/sys_common/backtrace.rs:138:18
  13:     0x563571da5499 - rust_begin_unwind
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:584:5
  14:     0x563571d8e333 - core::panicking::panic_fmt::h366d3a309ae17c94
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/panicking.rs:143:14
  15:     0x563571d8e1fd - core::panicking::panic::h8705e81f284be8a5
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/panicking.rs:48:5
  16:     0x563571d8e97a - <playground::MyStruct as core::fmt::Display>::fmt::h6f90f32f95d43e3a
                               at /playground/src/main.rs:7:9
  17:     0x563571dbd85c - core::fmt::write::hba4337c43d992f49
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/fmt/mod.rs:1194:17
  18:     0x563571da560c - core::fmt::Write::write_fmt::h3214023ca791c445
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/fmt/mod.rs:186:9
  19:     0x563571da560c - std::panicking::begin_panic_handler::PanicPayload::fill::{{closure}}::hb23d5bd8873a190f
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:550:22
  20:     0x563571da560c - core::option::Option<T>::get_or_insert_with::hbda74080d40f0cf4
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/option.rs:1542:49
  21:     0x563571da560c - std::panicking::begin_panic_handler::PanicPayload::fill::hb290995d7f51538f
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:548:13
  22:     0x563571da560c - <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get::hfba56e16158aca28
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:566:13
  23:     0x563571da5870 - std::panicking::rust_panic_with_hook::habfdcc2e90f9fd4c
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:697:34
  24:     0x563571da5767 - std::panicking::begin_panic_handler::{{closure}}::he054b2a83a51d2cd
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:588:13
  25:     0x563571da4044 - std::sys_common::backtrace::__rust_end_short_backtrace::ha48b94ab49b30915
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/sys_common/backtrace.rs:138:18
  26:     0x563571da5499 - rust_begin_unwind
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:584:5
  27:     0x563571d8e333 - core::panicking::panic_fmt::h366d3a309ae17c94
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/panicking.rs:143:14
  28:     0x563571d8e9e0 - playground::main::h558e18b54abd4495
                               at /playground/src/main.rs:14:5
  29:     0x563571d8e8fb - core::ops::function::FnOnce::call_once::h570318ebb6504eb8
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/ops/function.rs:227:5
  30:     0x563571d8e5fe - std::sys_common::backtrace::__rust_begin_short_backtrace::he78760a9857bb664
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/sys_common/backtrace.rs:122:18
  31:     0x563571d8e691 - std::rt::lang_start::{{closure}}::hbf4c93d41c9dece9
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/rt.rs:145:18
  32:     0x563571d9fc6e - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::had4f69b3aefb47a8
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/ops/function.rs:259:13
  33:     0x563571d9fc6e - std::panicking::try::do_call::hf2ad5355fcafe775
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:492:40
  34:     0x563571d9fc6e - std::panicking::try::h0a63ac363423e61e
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:456:19
  35:     0x563571d9fc6e - std::panic::catch_unwind::h18088edcecb8693a
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panic.rs:137:14
  36:     0x563571d9fc6e - std::rt::lang_start_internal::{{closure}}::ha7dad166dc711761
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/rt.rs:128:48
  37:     0x563571d9fc6e - std::panicking::try::do_call::hda0c61bf3a57d6e6
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:492:40
  38:     0x563571d9fc6e - std::panicking::try::hbc940e68560040a9
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panicking.rs:456:19
  39:     0x563571d9fc6e - std::panic::catch_unwind::haed0df2aeb3fa368
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/panic.rs:137:14
  40:     0x563571d9fc6e - std::rt::lang_start_internal::h9c06694362b5b80c
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/rt.rs:128:20
  41:     0x563571d8e660 - std::rt::lang_start::hdacf7ba74a6dd5d1
                               at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/std/src/rt.rs:144:17
  42:     0x563571d8ea0c - main
  43:     0x7fc16c471083 - __libc_start_main
  44:     0x563571d8e52e - _start
  45:                0x0 - <unknown>
thread panicked while panicking. aborting.
