plain
........................................................................................ 3344/4111
........................................................................................ 3432/4111
........................................................................................ 3520/4111
........................................................................................ 3608/4111
................................................................F..................F.... 3696/4111
........i........................i........................i........................i.... 3872/4111
....................i.....................i........................i.................... 3960/4111
....i........................i........................i................................. 4048/4111
...............................................................
---
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `None`,
 right: `Some(17)`', src/str/mod.rs:8:1
stack backtrace:
   0:     0x55ac6f8a8565 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h328e207c0fd2205c
   1:     0x55ac6f8c4088 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x55ac6f8a6961 - std::io::Write::write_fmt::hf1d0b0080eeddb6d
   3:     0x55ac6f8a8375 - std::sys_common::backtrace::print::h031dcf236c316efb
   4:     0x55ac6f8a9b94 - std::panicking::default_hook::{{closure}}::h9f518d39880907e1
   5:     0x55ac6f8a988b - std::panicking::default_hook::h689a8190a575275d
   6:     0x55ac6f8aa064 - std::panicking::rust_panic_with_hook::h527f6e576e1297bc
   7:     0x55ac6f8a9f59 - std::panicking::begin_panic_handler::{{closure}}::hbd4fa4e569c60071
   8:     0x55ac6f8a8a16 - std::sys_common::backtrace::__rust_end_short_backtrace::hb8d4f72b868865f5
   9:     0x55ac6f8a9c22 - rust_begin_unwind
  10:     0x55ac6f8858c3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  11:     0x55ac6f885b3f - core::panicking::assert_failed_inner::he3c123340f62e981
  12:     0x55ac6f888666 - core::panicking::assert_failed::h3829da7a2ce7f5ec
  13:     0x55ac6f889839 - rust_out::main::_doctest_main_src_str_mod_rs_1144_0::hfc8d2ddc1b6481b5
  14:     0x55ac6f889666 - rust_out::main::hdc6ca12ef188ae7f
  15:     0x55ac6f8866c3 - core::ops::function::FnOnce::call_once::hdc0cadfae62e0a97
  16:     0x55ac6f8861e9 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1c29b649872655be
  17:     0x55ac6f886239 - std::rt::lang_start::{{closure}}::h7bb98dbf9a7729bd
  18:     0x55ac6f8a4ab7 - std::rt::lang_start_internal::he0ea8d812d2478d4
  19:     0x55ac6f886217 - std::rt::lang_start::h06699a4618ecc88e
  20:     0x55ac6f889855 - main
  21:     0x7fd676b02d90 - <unknown>
  23:     0x55ac6f885df5 - _start
  24:                0x0 - <unknown>


---
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `None`,
 right: `Some(24)`', src/str/mod.rs:8:1
stack backtrace:
   0:     0x55dc1635a4a5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h328e207c0fd2205c
   1:     0x55dc16375fc8 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x55dc163588a1 - std::io::Write::write_fmt::hf1d0b0080eeddb6d
   3:     0x55dc1635a2b5 - std::sys_common::backtrace::print::h031dcf236c316efb
   4:     0x55dc1635bad4 - std::panicking::default_hook::{{closure}}::h9f518d39880907e1
   5:     0x55dc1635b7cb - std::panicking::default_hook::h689a8190a575275d
   6:     0x55dc1635bfa4 - std::panicking::rust_panic_with_hook::h527f6e576e1297bc
   7:     0x55dc1635be99 - std::panicking::begin_panic_handler::{{closure}}::hbd4fa4e569c60071
   8:     0x55dc1635a956 - std::sys_common::backtrace::__rust_end_short_backtrace::hb8d4f72b868865f5
   9:     0x55dc1635bb62 - rust_begin_unwind
  10:     0x55dc163378c3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  11:     0x55dc16337b3f - core::panicking::assert_failed_inner::he3c123340f62e981
  12:     0x55dc1633a686 - core::panicking::assert_failed::h3829da7a2ce7f5ec
  13:     0x55dc1633b779 - rust_out::main::_doctest_main_src_str_mod_rs_1192_0::hd8070e055e5e7acb
  14:     0x55dc1633b5a6 - rust_out::main::hdc6ca12ef188ae7f
  15:     0x55dc16338783 - core::ops::function::FnOnce::call_once::hdc0cadfae62e0a97
  16:     0x55dc163382a9 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1c29b649872655be
  17:     0x55dc163382f9 - std::rt::lang_start::{{closure}}::h7bb98dbf9a7729bd
  18:     0x55dc163569f7 - std::rt::lang_start_internal::he0ea8d812d2478d4
  19:     0x55dc163382d7 - std::rt::lang_start::h06699a4618ecc88e
  21:     0x7fcc75635d90 - <unknown>
  22:     0x7fcc75635e40 - __libc_start_main
  23:     0x55dc16337df5 - _start
  24:                0x0 - <unknown>
---
    src/str/mod.rs - str::str::rfind (line 1192)

test result: FAILED. 4072 passed; 2 failed; 37 ignored; 0 measured; 0 filtered out; finished in 47.60s

error: doctest failed, to rerun pass `-p core --doc`
