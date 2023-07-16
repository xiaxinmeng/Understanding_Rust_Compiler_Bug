
stack backtrace:
   0:     0x7ff156b6ae13 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h1c4138dd2563046a
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7ff156b622c4 - std::sys_common::backtrace::_print::h1a74a957606bf27f
                               at libstd/sys_common/backtrace.rs:71
   2:     0x7ff156b67a1d - std::panicking::default_hook::{{closure}}::h5e80532a04e672b0
                               at libstd/sys_common/backtrace.rs:59
                               at libstd/panicking.rs:380
   3:     0x7ff156b67722 - std::panicking::default_hook::hc600898703eb7810
                               at libstd/panicking.rs:396
   4:     0x7ff156b67e90 - std::panicking::rust_panic_with_hook::h715939b5e3315e96
                               at libstd/panicking.rs:576
   5:     0x7ff156b67d4e - std::panicking::begin_panic::h3c2c20be4873dd6d
                               at libstd/panicking.rs:537
   6:     0x7ff156b67c49 - std::panicking::begin_panic_fmt::hd4d3384b7858d996
                               at libstd/panicking.rs:521
   7:     0x7ff156b67bd2 - rust_begin_unwind
                               at libstd/panicking.rs:497
   8:     0x7ff156bccc40 - core::panicking::panic_fmt::hfa35666025eb7859
                               at libcore/panicking.rs:71
   9:     0x7ff156bce52b - core::str::slice_error_fail::h49104999cfa2d62a
                               at libcore/str/mod.rs:0
  10:     0x5623f323f853 - core::str::traits::<impl core::slice::SliceIndex<str> for core::ops::range::Range<usize>>::index::{{closure}}::ha9af485fcde299d3
                               at /checkout/src/libcore/str/mod.rs:1892
  11:     0x5623f331ecdb - rustdoc::html::markdown::find_testable_code::h26c37b3d5ca10f4f
                               at /checkout/src/libcore/option.rs:376
                               at /checkout/src/libcore/str/mod.rs:1892
                               at /checkout/src/libcore/str/mod.rs:1666
                               at librustdoc/html/markdown.rs:1033
  12:     0x5623f3379382 - rustdoc::test::run::hfa8057775d16884a
                               at librustdoc/test.rs:686
                               at librustdoc/test.rs:132
  13:     0x5623f338dd26 - rustdoc::main_args::h83fb12837160f46e
                               at librustdoc/lib.rs:446
  14:     0x5623f31f0211 - std::sys_common::backtrace::__rust_begin_short_backtrace::hcf4c4e6980ea96f8
                               at librustdoc/lib.rs:110
                               at /checkout/src/libcore/option.rs:404
                               at librustdoc/lib.rs:110
                               at /checkout/src/libstd/sys_common/backtrace.rs:136
  15:     0x5623f31f1a3d - std::panicking::try::do_call::h69b4a89faaea0911
                               at /checkout/src/libstd/thread/mod.rs:406
                               at /checkout/src/libstd/panic.rs:293
                               at /checkout/src/libstd/panicking.rs:479
  16:     0x7ff156b80bce - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  17:     0x5623f3241bc9 - <F as alloc::boxed::FnBox<A>>::call_box::hd33f01d4aed2432f
                               at /checkout/src/libstd/panicking.rs:458
                               at /checkout/src/libstd/panic.rs:358
                               at /checkout/src/libstd/thread/mod.rs:405
                               at /checkout/src/liballoc/boxed.rs:788
  18:     0x7ff156b77e5b - std::sys::unix::thread::Thread::new::thread_start::h9f8e50b1e2b0d243
                               at /checkout/src/liballoc/boxed.rs:798
                               at libstd/sys_common/thread.rs:24
                               at libstd/sys/unix/thread.rs:90
  19:     0x7ff1568e36a6 - <unknown>
  20:     0x7ff15640bc5e - __clone
  21:                0x0 - <unknown>
