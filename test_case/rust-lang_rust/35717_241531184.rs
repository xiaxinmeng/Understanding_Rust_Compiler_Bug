
/home/hugh/rust/src/librustc_driver/driver.rs:1278: line longer than 100 chars
/home/hugh/rust/src/librustc_driver/driver.rs:1279: line longer than 100 chars
thread 'main' panicked at 'some tidy checks failed', /home/hugh/rust/src/tools/tidy/src/main.rs:53
stack backtrace:
   1:     0x2b62dda2d17f - std::sys::backtrace::tracing::imp::write::hf504bc08f2a3011f
   2:     0x2b62dda3b4eb - std::panicking::default_hook::_{{closure}}::h4628a636a996f866
   3:     0x2b62dda3b11d - std::panicking::default_hook::hf5d79ad4f1ead81e
   4:     0x2b62dda006a3 - std::panicking::rust_panic_with_hook::h9a3925e37821d434
   5:     0x2b62dd508f9f - std::panicking::begin_panic::hf0f914ab539f4b11
   6:     0x2b62dd50833a - tidy::main::h98b9055b63937e8d
   7:     0x2b62dda4a50b - __rust_try
   8:     0x2b62dda4a4ae - __rust_maybe_catch_panic
   9:     0x2b62dda3a547 - std::rt::lang_start::h30b674bef4ff14e2
  10:     0x2b62de0eaf44 - __libc_start_main
  11:     0x2b62dd5055f8 - <unknown>
  12:                0x0 - <unknown>
make: *** [tidy] Error 101
