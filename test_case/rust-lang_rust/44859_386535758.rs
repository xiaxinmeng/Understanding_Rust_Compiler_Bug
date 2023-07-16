
amnesia@amnesia:~$ RUST_BACKTRACE=1 ./panic
thread 'main' panicked at 'test', panic.rs:2
stack backtrace:
   1:     0x590200a6594a - std::sys::imp::backtrace::tracing::imp::write::h917062bce4ff48c3
                        at /build/rustc-1.14.0+dfsg1/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x590200a68a7f - std::panicking::default_hook::{{closure}}::h0bacac31b5ed1870
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:247
   3:     0x590200a6747c - std::panicking::default_hook::h5897799da33ece67
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:263
   4:     0x590200a67a77 - std::panicking::rust_panic_with_hook::h109e116a3a861224
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:451
   5:     0x590200a60896 - std::panicking::begin_panic::h634e2b37a96f78d4
   6:     0x590200a609ef - panic::main::ha800225901e63f47
   7:     0x590200a703ea - __rust_maybe_catch_panic
                        at /build/rustc-1.14.0+dfsg1/src/libpanic_unwind/lib.rs:97
   8:     0x590200a68255 - std::rt::lang_start::hd661476ce2fc2931
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:332
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panic.rs:351
                        at /build/rustc-1.14.0+dfsg1/src/libstd/rt.rs:57
   9:     0x590200a60a19 - main
  10:     0x7859a3fe72e0 - __libc_start_main
  11:     0x590200a606e9 - _start
  12:                0x0 - <unknown>
