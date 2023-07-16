
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "\xFF.rs"', libcore/result.rs:945:5
stack backtrace:
   0:        0x10ac6f81f - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h8f55de8450e228eb
   1:        0x10ac57f6a - std::sys_common::backtrace::print::hba2509af7942751e
   2:        0x10ac751f3 - std::panicking::default_hook::{{closure}}::h4acff16463eacf92
   3:        0x10ac74f7c - std::panicking::default_hook::h6c30345eb076443e
   4:        0x10ac758e7 - std::panicking::rust_panic_with_hook::hd3e277e0f6d3bf67
   5:        0x10ac7548c - std::panicking::continue_panic_fmt::ha430a2f10a3dda0e
   6:        0x10ac75378 - rust_begin_unwind
   7:        0x10acb8331 - core::panicking::panic_fmt::heebb61c9f37aa947
   8:        0x10ac6a67d - core::result::unwrap_failed::h59aff86f706786bc
   9:        0x10ac73b2b - <std::env::Args as core::iter::iterator::Iterator>::next::hc21880c5b940ecc5
  10:        0x10a866aa4 - rustup_init::proxy_mode::main::h82303ca0b34607e1
  11:        0x10a842582 - rustup_init::main::h2cd75d5bbd385c41
  12:        0x10a83ba75 - std::rt::lang_start::{{closure}}::hc59e8d235996c800
  13:        0x10ac752f7 - std::panicking::try::do_call::h075d1fe7e0e0cdba
  14:        0x10ac8134e - __rust_maybe_catch_panic
  15:        0x10ac62d6c - std::rt::lang_start_internal::h59f4ab4861dc5e32
  16:        0x10a8429eb - main
