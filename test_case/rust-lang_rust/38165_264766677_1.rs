
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ()', src/libcore/result.rs:837
stack backtrace:
   1:     0x56381e42288e - std::sys::imp::backtrace::tracing::imp::write::hfc2edb670e5eda97
   2:     0x56381e424b91 - std::panicking::default_hook::{{closure}}::hc66a547fab0b4d38
   3:     0x56381e424776 - std::panicking::default_hook::h7fba1b2f69474bdc
   4:     0x56381e424f78 - std::panicking::rust_panic_with_hook::h5d3597668c9f0035
   5:     0x56381e424e12 - std::panicking::begin_panic::hfaa38fdbc7d103f6
   6:     0x56381e424d50 - std::panicking::begin_panic_fmt::h69e0397ed8fc5362
   7:     0x56381e424cd1 - rust_begin_unwind
   8:     0x56381e46db2f - core::panicking::panic_fmt::h2ba266031787cf7b
   9:     0x56381e41cc55 - core::result::unwrap_failed::h0a78b8b4914ea99c
                               at /home/yamakaky/dev/rust/rust/src/libcore/macros.rs:29
  10:     0x56381e41c99b - <core::result::Result<T, E>>::unwrap::h2f7c43c0e2d4b7cd
                               at /home/yamakaky/dev/rust/rust/src/libcore/result.rs:737
  11:     0x56381e41ccd8 - t::main::h86d9b74d94eb7fff
                               at /tmp/pote/t.rs:2
  12:     0x56381e42f0f6 - __rust_maybe_catch_panic
  13:     0x56381e4255a8 - std::rt::lang_start::ha33d35ddc5bfd7d2
  14:     0x56381e41cd12 - main
  15:     0x7f6dc966c290 - __libc_start_main
  16:     0x56381e41c7f9 - _start
