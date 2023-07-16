
2020-08-02T05:58:52.1016977Z running 14 tests
2020-08-02T05:58:52.1017997Z test freestanding::tests::assume ... ok
2020-08-02T05:58:52.1019489Z test freestanding::tests::compressing_stringify ... ok
2020-08-02T05:58:52.1021633Z test freestanding::tests::compressing_include_str ... ok
2020-08-02T05:58:52.1022991Z test freestanding::tests::global_ctor ... ok
2020-08-02T05:58:52.1023776Z test freestanding::tests::const_default ... ok
2020-08-02T05:58:52.1026345Z test freestanding::tests::result_swap ... ok
2020-08-02T05:58:52.1027920Z test freestanding::tests::guard ... ok
2020-08-02T05:58:52.1029857Z test freestanding::tests::utf16 ... ok
2020-08-02T05:58:52.1031776Z test freestanding::tests::scope_exit ... ok
2020-08-02T05:58:52.1034426Z test full_featured_os::argh::tests::from_custom_env_as_result ... ok
2020-08-02T05:58:52.1058549Z thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/libtest/lib.rs:473:32
2020-08-02T05:58:52.1059340Z stack backtrace:
2020-08-02T05:58:52.1259107Z    0:     0x55ee97a8b2b5 - backtrace::backtrace::libunwind::trace::h75aedf5f78e5147f
2020-08-02T05:58:52.1260563Z                                at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
2020-08-02T05:58:52.1261725Z    1:     0x55ee97a8b2b5 - backtrace::backtrace::trace_unsynchronized::h18fb73c9ac9ae753
2020-08-02T05:58:52.1262530Z                                at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
2020-08-02T05:58:52.1263603Z    2:     0x55ee97a8b2b5 - std::sys_common::backtrace::_print_fmt::h65f97470ff13ec84
2020-08-02T05:58:52.1264093Z                                at src/libstd/sys_common/backtrace.rs:78
2020-08-02T05:58:52.1265013Z    3:     0x55ee97a8b2b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hee061c54ddc9f024
2020-08-02T05:58:52.1265475Z                                at src/libstd/sys_common/backtrace.rs:59
2020-08-02T05:58:52.1368834Z    4:     0x55ee97ab2e2c - core::fmt::write::hfbd2baad61ed21a8
2020-08-02T05:58:52.1369441Z                                at src/libcore/fmt/mod.rs:1117
2020-08-02T05:58:52.1370425Z    5:     0x55ee97a87b92 - std::io::Write::write_fmt::h72f9bd227f40dc62
2020-08-02T05:58:52.1371096Z                                at src/libstd/io/mod.rs:1508
2020-08-02T05:58:52.1372016Z    6:     0x55ee97a8da10 - std::sys_common::backtrace::_print::h2d2cd8fe02feb5fa
2020-08-02T05:58:52.1372504Z                                at src/libstd/sys_common/backtrace.rs:62
2020-08-02T05:58:52.1373628Z    7:     0x55ee97a8da10 - std::sys_common::backtrace::print::h801b12991252ba7c
2020-08-02T05:58:52.1374488Z                                at src/libstd/sys_common/backtrace.rs:49
2020-08-02T05:58:52.1375698Z    8:     0x55ee97a8da10 - std::panicking::default_hook::{{closure}}::h25fc1fbf3b63b5c8
2020-08-02T05:58:52.1376078Z                                at src/libstd/panicking.rs:198
2020-08-02T05:58:52.1376842Z    9:     0x55ee97a8d75c - std::panicking::default_hook::h62c897957a5e0f26
2020-08-02T05:58:52.1377185Z                                at src/libstd/panicking.rs:217
2020-08-02T05:58:52.1378268Z   10:     0x55ee97a8e053 - std::panicking::rust_panic_with_hook::hb8a276f163c59810
2020-08-02T05:58:52.1387914Z                                at src/libstd/panicking.rs:526
2020-08-02T05:58:52.1389070Z   11:     0x55ee97a8dc4b - rust_begin_unwind
2020-08-02T05:58:52.1389536Z                                at src/libstd/panicking.rs:437
2020-08-02T05:58:52.1390265Z   12:     0x55ee97ab17c1 - core::panicking::panic_fmt::h9cc57011b345cfad
2020-08-02T05:58:52.1390591Z                                at src/libcore/panicking.rs:85
2020-08-02T05:58:52.1391254Z   13:     0x55ee97ab15e3 - core::option::expect_none_failed::h10abb5a6aef32df8
2020-08-02T05:58:52.1391557Z                                at src/libcore/option.rs:1273
2020-08-02T05:58:52.1492464Z   14:     0x55ee97a66a1e - core::result::Result<T,E>::unwrap::hf701633582f46c23
2020-08-02T05:58:52.1494631Z                                at /rustc/d6953df14657f5932270ad2b33bccafe6f39fad4/src/libcore/result.rs:1005
2020-08-02T05:58:52.1495684Z   15:     0x55ee97a66a1e - test::run_test::run_test_inner::hb4e0cf3cefb35bf3
2020-08-02T05:58:52.1496425Z                                at src/libtest/lib.rs:473
2020-08-02T05:58:52.1497186Z   16:     0x55ee97a6480e - test::run_test::h51caf8e89b554a56
2020-08-02T05:58:52.1497621Z                                at src/libtest/lib.rs:505
2020-08-02T05:58:52.1498386Z   17:     0x55ee97a526da - test::run_tests::h81fcb787d3f44144
2020-08-02T05:58:52.1499208Z                                at src/libtest/lib.rs:299
2020-08-02T05:58:52.1500080Z   18:     0x55ee97a526da - test::console::run_tests_console::h93fbaddef781791c
2020-08-02T05:58:52.1500718Z                                at src/libtest/console.rs:280
2020-08-02T05:58:52.1501881Z   19:     0x55ee97a60ada - test::test_main::heb8bd877a723c55d
2020-08-02T05:58:52.1515862Z                                at src/libtest/lib.rs:120
2020-08-02T05:58:52.1517005Z   20:     0x55ee97a6200d - test::test_main_static::h05dbfcdf8166a1c5
2020-08-02T05:58:52.1517464Z                                at src/libtest/lib.rs:139
2020-08-02T05:58:52.1518205Z   21:     0x55ee97a2bd26 - hut::main::hd669fce343160431
2020-08-02T05:58:52.1519005Z   22:     0x55ee97a2598b - std::rt::lang_start::{{closure}}::h6e6e62a5244ee3dd
2020-08-02T05:58:52.1519440Z                                at /rustc/d6953df14657f5932270ad2b33bccafe6f39fad4/src/libstd/rt.rs:67
2020-08-02T05:58:52.1520218Z   23:     0x55ee97a8e423 - std::rt::lang_start_internal::{{closure}}::hbd178e645b70b347
2020-08-02T05:58:52.1520858Z                                at src/libstd/rt.rs:52
2020-08-02T05:58:52.1521694Z   24:     0x55ee97a8e423 - std::panicking::try::do_call::hd9e76f93421bce23
2020-08-02T05:58:52.1522165Z                                at src/libstd/panicking.rs:348
2020-08-02T05:58:52.1522995Z   25:     0x55ee97a8e423 - std::panicking::try::h6776eea046a81bd7
2020-08-02T05:58:52.1523461Z                                at src/libstd/panicking.rs:325
2020-08-02T05:58:52.1524447Z   26:     0x55ee97a8e423 - std::panic::catch_unwind::hd9dfb4dd4c6fb7d1
2020-08-02T05:58:52.1524884Z                                at src/libstd/panic.rs:394
2020-08-02T05:58:52.1526010Z   27:     0x55ee97a8e423 - std::rt::lang_start_internal::h47278b515c002423
2020-08-02T05:58:52.1526433Z                                at src/libstd/rt.rs:51
2020-08-02T05:58:52.1527167Z   28:     0x55ee97a25967 - std::rt::lang_start::h0f0f6db08b6fd5bc
2020-08-02T05:58:52.1527623Z                                at /rustc/d6953df14657f5932270ad2b33bccafe6f39fad4/src/libstd/rt.rs:67
2020-08-02T05:58:52.1528408Z   29:     0x55ee97a2bd5a - main
2020-08-02T05:58:52.1529222Z   30:     0x7fd5af538b97 - __libc_start_main
2020-08-02T05:58:52.1530062Z   31:     0x55ee97a1954a - _start
2020-08-02T05:58:52.1530667Z   32:                0x0 - <unknown>
2020-08-02T05:58:52.1532853Z error: test failed, to rerun pass '--lib'
2020-08-02T05:58:52.1557798Z ##[error]The process '/usr/share/rust/.cargo/bin/cargo' failed with exit code 101
2020-08-02T05:58:52.1633132Z Post job cleanup.
2020-08-02T05:58:52.2761660Z [command]/usr/bin/git version
2020-08-02T05:58:52.2825424Z git version 2.27.0
2020-08-02T05:58:52.2868383Z [command]/usr/bin/git config --local --name-only --get-regexp core\.sshCommand
2020-08-02T05:58:52.2905666Z [command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :
2020-08-02T05:58:52.3185655Z [command]/usr/bin/git config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
2020-08-02T05:58:52.3218314Z http.https://github.com/.extraheader
2020-08-02T05:58:52.3228570Z [command]/usr/bin/git config --local --unset-all http.https://github.com/.extraheader
2020-08-02T05:58:52.3268023Z [command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
2020-08-02T05:58:52.3586561Z Cleaning up orphan processes
