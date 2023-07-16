
thread 'main' panicked at 'explicit panic', t.rs:2
stack backtrace:
   0:     0x562b31ac45f8 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::hf65740f571b3b062
                               at /home/yamakaky/dev/rust/rust/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x562b31af430e - std::sys_common::backtrace::_print::h49fe0d471b73c20e
                               at /home/yamakaky/dev/rust/rust/src/libstd/sys_common/backtrace.rs:71
   2:     0x562b31af351e - std::sys_common::backtrace::print::hd65a646efeb9e475
                               at /home/yamakaky/dev/rust/rust/src/libstd/sys_common/backtrace.rs:60
   3:     0x562b31ac704e - std::panicking::default_hook::{{closure}}::h25191b25fd3a12b5
                               at /home/yamakaky/dev/rust/rust/src/libstd/panicking.rs:355
   4:     0x562b31ac6b1c - std::panicking::default_hook::hf7e171cc5ec0b024
                               at /home/yamakaky/dev/rust/rust/src/libstd/panicking.rs:371
   5:     0x562b31ac795c - std::panicking::rust_panic_with_hook::h01e0c787710839ce
                               at /home/yamakaky/dev/rust/rust/src/libstd/panicking.rs:549
   6:     0x562b31ab5343 - std::panicking::begin_panic::h67ddf8e748228f4b
                               at /home/yamakaky/dev/rust/rust/src/libstd/panicking.rs:511
   7:     0x562b31ab5852 - t::main::hf31397ea67eea730
                               at /tmp/t.rs:2
   8:     0x562b31ac7e85 - core::ops::FnOnce::call_once::h97136135af0c9b86
   9:     0x562b31ac737a - std::panicking::try::do_call::h0b3ba9923f346cc7
                               at /home/yamakaky/dev/rust/rust/src/libstd/panicking.rs:454
  10:     0x562b31b08347 - __rust_try
  11:     0x562b31b08227 - __rust_maybe_catch_panic
                               at /home/yamakaky/dev/rust/rust/src/libpanic_unwind/lib.rs:98
  12:     0x562b31ac7167 - std::panicking::try::h2b94160ac273c42a
                               at /home/yamakaky/dev/rust/rust/src/libstd/panicking.rs:433
  13:     0x562b31adc66a - std::panic::catch_unwind::h8c1f7b1121d92ea1
                               at /home/yamakaky/dev/rust/rust/src/libstd/panic.rs:361
  14:     0x562b31adf67e - std::rt::lang_start::h29d9bdfd34a5eab2
                               at /home/yamakaky/dev/rust/rust/src/libstd/rt.rs:57
  15:     0x562b31ab5892 - main
  16:     0x7feb00a57290 - __libc_start_main
  17:     0x562b31ab5169 - _start
  18:                0x0 - <unknown>
