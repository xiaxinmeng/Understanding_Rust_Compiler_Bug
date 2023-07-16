
$ RUST_BACKTRACE=1 rustc -Z unstable-options --pretty normal main.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:330
stack backtrace:
   1:     0x7fa6afcc9780 - sys::backtrace::tracing::imp::write::h95a06036b37126f3Inu
   2:     0x7fa6afcd0425 - panicking::default_handler::_<closure>::closure.42754
   3:     0x7fa6afccffb3 - panicking::default_handler::h52ff76369879c06eaFy
   4:     0x7fa6afc997c6 - sys_common::unwind::begin_unwind_inner::ha2d7340df818fe5efgt
   5:     0x7fa6afc9a188 - sys_common::unwind::begin_unwind_fmt::hbf84897d960dd3falft
   6:     0x7fa6afcc7391 - rust_begin_unwind
   7:     0x7fa6afd2295f - panicking::panic_fmt::h429a8e624209fa72nYL
   8:     0x7fa6afd23458 - panicking::panic::h405d26906251684dUWL
   9:     0x7fa6abd8029a - print::pprust::State<'a>::bclose_maybe_open::h691140728605ea9dfM1
  10:     0x7fa6abd8b476 - print::pprust::State<'a>::print_block_maybe_unclosed::h0abaae9e5f2e58d5RI3
  11:     0x7fa6abd76b4f - print::pprust::State<'a>::print_item::ha28389c25b5ba8d8wl2
  12:     0x7fa6abd686de - print::pprust::print_crate::h7ba34fcca613a873dF0
  13:     0x7fa6b028dee7 - pretty::pretty_print_input::_<closure>::closure.35080
  14:     0x7fa6b01ab160 - pretty::pretty_print_input::hfe9a6ab02ec6f263i5b
  15:     0x7fa6b01a0edc - run_compiler::he8ecf6cfff34818aexc
  16:     0x7fa6b019e271 - sys_common::unwind::try::try_fn::h6472701518541106374
  17:     0x7fa6afcc7328 - __rust_try
  18:     0x7fa6afcbed8b - sys_common::unwind::try::inner_try::hef60513bede8266eNct
  19:     0x7fa6b019e5d0 - boxed::F.FnBox<A>::call_box::h12686579556207503277
  20:     0x7fa6afcce9a3 - sys::thread::Thread::new::thread_start::h046316123445d4346Bx
  21:     0x7fa6a9468283 - start_thread
  22:     0x7fa6af94d74c - clone
  23:                0x0 - <unknown>

$ rustc --version                                                   
rustc 1.8.0-dev (8ff48fea1 2016-01-23)
