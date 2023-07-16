plain
failures:

---- [ui] ui/issues/issue-54477-reduced-2.rs stdout ----

error: test run failed!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54477-reduced-2/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: size.is_power_of_two()', /checkout/library/alloc/src/collections/vec_deque/mod.rs:2796:5
   0:     0x7ff492fda2dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2a1e2ea0dbac18be
   0:     0x7ff492fda2dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2a1e2ea0dbac18be
   1:     0x7ff493041b2f - core::fmt::write::hbb453a4d778e11ff
   2:     0x7ff492fc9fe1 - std::io::Write::write_fmt::hddacc509a927e885
   3:     0x7ff492fda0fb - std::sys_common::backtrace::print::hc7cafe14194765c9
   4:     0x7ff492fdd8e4 - std::panicking::default_hook::{{closure}}::h303a5f48fa37ba9b
   5:     0x7ff492fdd49d - std::panicking::default_hook::h13d2375def55a5ec
   6:     0x7ff492fddf1e - std::panicking::rust_panic_with_hook::hb02a5190582d8d97
   7:     0x7ff492fdddb9 - std::panicking::begin_panic_handler::{{closure}}::h5f39923135a8e24c
   8:     0x7ff492fda7f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h58723cb57af09663
   9:     0x7ff492fddad9 - rust_begin_unwind
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  10:     0x7ff492f8ba43 - core::panicking::panic_fmt::h91b83c14b5eee854
  11:     0x7ff492f8b90d - core::panicking::panic::hce6398d25399b8ab
  12:     0x56077aa524b7 - <&T as core::fmt::Debug>::fmt::h8deea6b2ae39abc6
  13:     0x7ff493041b2f - core::fmt::write::hbb453a4d778e11ff
  14:     0x7ff492fddc7f - <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get::hdd2c384177ebdbcc
  15:     0x7ff492fddf09 - std::panicking::rust_panic_with_hook::hb02a5190582d8d97
  16:     0x7ff492fdddf7 - std::panicking::begin_panic_handler::{{closure}}::h5f39923135a8e24c
  17:     0x7ff492fda7f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h58723cb57af09663
  18:     0x7ff492fddad9 - rust_begin_unwind
  19:     0x7ff492f8ba43 - core::panicking::panic_fmt::h91b83c14b5eee854
  20:     0x7ff49303e008 - core::panicking::assert_failed_inner::h84cca991f6d8b130
  21:     0x56077aa520a4 - core::panicking::assert_failed::h18fe13fea6b127bd
  22:     0x56077aa53b6a - issue_54477_reduced_2::main::h2574c7f9ed9a5d76
  23:     0x56077aa522d3 - std::sys_common::backtrace::__rust_begin_short_backtrace::h517d0eb66e88748e
  24:     0x56077aa522e9 - std::rt::lang_start::{{closure}}::hf30fa9b20627d6ee
  25:     0x7ff492fb6cdb - std::rt::lang_start_internal::hfda263f9ed9a7d1f
  26:     0x56077aa53f02 - main
  27:     0x7ff492d330b3 - __libc_start_main
  28:     0x56077aa5220e - _start
  29:                0x0 - <unknown>
------------------------------------------



