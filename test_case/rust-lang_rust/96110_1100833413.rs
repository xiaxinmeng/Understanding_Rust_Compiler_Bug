plain
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `None`,
 right: `Some(1)`', src/num/mod.rs:8:1
stack backtrace:
   0:     0x56388501e5ec - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h247a96780867db43
   1:     0x563885039588 - core::fmt::write::h93aed11279643495
   2:     0x56388501c4e1 - std::io::Write::write_fmt::hb8987a2746accb53
   3:     0x56388501ff1e - std::panicking::default_hook::{{closure}}::hf74799afdcde712d
   4:     0x56388501fb4c - std::panicking::default_hook::hebda680c143b8d90
   5:     0x563885020541 - std::panicking::rust_panic_with_hook::h94317094a36dacaf
   6:     0x5638850203e7 - std::panicking::begin_panic_handler::{{closure}}::ha3e89f2d80b54b32
   7:     0x56388501eb04 - std::sys_common::backtrace::__rust_end_short_backtrace::h44ed2f3a4646e6cb
   8:     0x5638850200c9 - rust_begin_unwind
   9:     0x563884ffddb3 - core::panicking::panic_fmt::h985b968f854695bc
  10:     0x563885038868 - core::panicking::assert_failed_inner::h0b7c2788631f5941
  11:     0x563884ffe356 - core::panicking::assert_failed::h94536c3e63cd1d9d
  12:     0x563884ffe5a7 - rust_out::main::_doctest_main_src_num_mod_rs_342_0::h273da6e4495e90f9
  13:     0x563884ffe4e6 - rust_out::main::h96a65816d11aeaee
  14:     0x563884ffe2b3 - core::ops::function::FnOnce::call_once::h7d7481578004f638
  15:     0x563884ffe079 - std::sys_common::backtrace::__rust_begin_short_backtrace::hddd05b25c94f476f
  16:     0x563884ffe0e9 - std::rt::lang_start::{{closure}}::hfe6094dd5022f654
  17:     0x56388501a58b - std::rt::lang_start_internal::h2f99b2008435c2cb
  18:     0x563884ffe0d1 - std::rt::lang_start::h3ee97cca994dda0d
  19:     0x563884ffe653 - main
  20:     0x7f17583310b3 - __libc_start_main
  21:     0x563884ffdfae - _start
  22:                0x0 - <unknown>


failures:
    src/num/mod.rs - num::u8::to_ascii_digit (line 342)
