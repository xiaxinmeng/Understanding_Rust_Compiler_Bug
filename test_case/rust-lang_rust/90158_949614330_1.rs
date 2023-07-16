
thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler\rustc_errors\src\lib.rs:1166:13
stack backtrace:
   0:     0x7ffc9fdf8bff - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4287336b592e4e30
   1:     0x7ffc9fe228ba - core::fmt::write::h8a2c40ddb66ccc71
   2:     0x7ffc9fdeb5b8 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::hd15f09e3fd3eac54
   3:     0x7ffc9fdfc356 - std::panicking::take_hook::hf14c76592f73c762
   4:     0x7ffc9fdfbe3c - std::panicking::take_hook::hf14c76592f73c762
   5:     0x7ffc6a2d47fe - <sha2::sha512::Sha384 as std::io::Write>::flush::h62d93fd0af78bc9c
   6:     0x7ffc9fdfcc69 - std::panicking::rust_panic_with_hook::hbc0e9c80ca88eac0
   7:     0x7ffc9fdfc70b - rust_begin_unwind
   8:     0x7ffc9fdf9537 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4287336b592e4e30
   9:     0x7ffc9fdfc669 - rust_begin_unwind
  10:     0x7ffc9fe58dd0 - core::panicking::panic_fmt::hc1b1ca620e7a2c9f
  11:     0x7ffc6eadb967 - <rustc_errors::registry::InvalidErrorCode as core::fmt::Debug>::fmt::habc00740dca1d786
  12:     0x7ffc6eae08b8 - rustc_errors::HandlerInner::delay_as_bug::hc436ffe89340a533
  13:     0x7ffc6eadc532 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h9bea109db35ed3b5
  14:     0x7ffc6a371ec4 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::heb70c7870704ee76
  15:     0x7ffc6a379f77 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::heb70c7870704ee76
  16:     0x7ffc6a2f600d - <tracing_subscriber::util::TryInitError as core::fmt::Display>::fmt::h7aa873eeea16c083
  17:     0x7ffc6a2f28a2 - rustc_driver::pretty::print_after_hir_lowering::hb7dd632ff4fb38d2
  18:     0x7ffc6a33eeb6 - <rustc_middle::ty::SymbolName as core::fmt::Debug>::fmt::h1c205b30aad4a3e6
  19:     0x7ffc6a2f9c23 - <tracing_subscriber::util::TryInitError as core::fmt::Display>::fmt::h7aa873eeea16c083
  20:     0x7ffc6a368d48 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::heb70c7870704ee76
  21:     0x7ffc9fe0932c - std::sys::windows::thread::Thread::new::he49a5cdb1d3d1cfc
  22:     0x7ffd0bf37c24 - BaseThreadInitThunk
  23:     0x7ffd0d4ad4d1 - RtlUserThreadStart
