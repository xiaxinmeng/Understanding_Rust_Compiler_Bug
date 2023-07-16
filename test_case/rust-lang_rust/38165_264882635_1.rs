
thread 'main' panicked at 'explicit panic', t.rs:2
stack backtrace:
   1:     0x559456964347 - std::sys::imp::backtrace::tracing::imp::write::hd29ea4b7d57abadd
   2:     0x559456969781 - std::panicking::default_hook::{{closure}}::h7cb71ec6c6050f50
   3:     0x559456968376 - std::panicking::default_hook::h7fba1b2f69474bdc
   4:     0x559456968978 - std::panicking::rust_panic_with_hook::h1b9f3a413c4c29f3
   5:     0x559456960263 - std::panicking::begin_panic::h5f62bd622a6e3525
                               at /home/yamakaky/dev/rust/rust/src/libstd/panicking.rs:417
   6:     0x559456960412 - t::test::hc6402ffe28ce1338
                               at /tmp/pote/t.rs:2
   7:     0x55945696042c - t::main::h86d9b74d94eb7fff
                               at /tmp/pote/t.rs:5
   8:     0x5594569710d6 - __rust_maybe_catch_panic
   9:     0x559456968fa8 - std::rt::lang_start::ha33d35ddc5bfd7d2
  10:     0x559456960472 - main
  11:     0x7f05f168c290 - __libc_start_main
  12:     0x559456960089 - _start
