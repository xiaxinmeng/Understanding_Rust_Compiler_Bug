
thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler\rustc_errors\src\lib.rs:974:13
stack backtrace:
   0:     0x7ffe7e17a24e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9b227e9f535b30ad
   1:     0x7ffe7e1a841b - core::fmt::write::h61d774f0fccca269
   2:     0x7ffe7e16bc68 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::hd02efffdeb825e77
   3:     0x7ffe7e17f86d - std::panicking::take_hook::h071d302a205c6652
   4:     0x7ffe7e17f448 - std::panicking::take_hook::h071d302a205c6652
   5:     0x7ffe26f58c27 - rustc_driver::report_ice::h5fb666ac0b140095
   6:     0x7ffe7e180380 - std::panicking::rust_panic_with_hook::h3a6bf3c946dc3fd3
   7:     0x7ffe7e17fe51 - rust_begin_unwind
   8:     0x7ffe7e17ab8f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9b227e9f535b30ad
   9:     0x7ffe7e17fda9 - rust_begin_unwind
  10:     0x7ffe7e17fd5c - std::panicking::begin_panic_fmt::h4544c6b6ce1d7abc
  11:     0x7ffe2b670484 - rustc_errors::HandlerInner::delay_as_bug::h14efc84023f1ac5a
  12:     0x7ffe2b66ba14 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::he0453085c4ee1c80
  13:     0x7ffe26f40194 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h9bf3232a4fe8dccd
  14:     0x7ffe26f45a90 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h9bf3232a4fe8dccd
  15:     0x7ffe26f4ad1a - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h9bf3232a4fe8dccd
  16:     0x7ffe26f62acc - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h33be7dbc37f75bef
  17:     0x7ffe26f5b2b4 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h33be7dbc37f75bef
  18:     0x7ffe26f6aa08 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h33be7dbc37f75bef
  19:     0x7ffe26f5d09e - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h33be7dbc37f75bef
  20:     0x7ffe26f6f883 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h33be7dbc37f75bef
  21:     0x7ffe26edbe8d - <sha1::Sha1 as std::io::Write>::flush::hc1a90b50a4949fda
  22:     0x7ffe7e190537 - std::sys::windows::thread::Thread::new::h010378fec97bafdf
  23:     0x7ffeb4867034 - BaseThreadInitThunk
  24:     0x7ffeb4b7d0d1 - RtlUserThreadStart
