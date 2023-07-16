
$ RUST_BACKTRACE=1 ./inf
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 0, message: "Success" } }', /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/result.rs:837
stack backtrace:
   1:     0x5620c17327ca - std::sys::imp::backtrace::tracing::imp::write::h3188f035833a2635
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x5620c1734f6f - std::panicking::default_hook::{{closure}}::h6385b6959a2dd25b
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:349
   3:     0x5620c1734b6e - std::panicking::default_hook::he4f3b61755d7fa95
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:365
   4:     0x5620c17353b7 - std::panicking::rust_panic_with_hook::hf00b8130f73095ec
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:553
   5:     0x5620c17351f4 - std::panicking::begin_panic::h6227f62cb2cdaeb4
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:515
   6:     0x5620c1735169 - std::panicking::begin_panic_fmt::h173eadd80ae64bec
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:499
   7:     0x5620c17350f7 - rust_begin_unwind
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:475
   8:     0x5620c175b11d - core::panicking::panic_fmt::h3b2d1e30090844ff
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/panicking.rs:69
   9:     0x5620c172cdc4 - core::result::unwrap_failed::h822856a25f2ebc35
  10:     0x5620c173cf0a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  11:     0x5620c172cf01 - <F as alloc::boxed::FnBox<A>>::call_box::hd0e958eef64c8c8a
  12:     0x5620c17343b4 - std::sys::imp::thread::Thread::new::thread_start::he018521f53b24939
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/liballoc/boxed.rs:615
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  13:     0x7f32984a66b9 - start_thread
  14:     0x7f3297fc682c - clone
  15:                0x0 - <unknown>
