
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 0, message: "Success" } }', src/libcore/result.rs:837
stack backtrace:
1:     0x5650bd0acada - std::sys::imp::backtrace::tracing::imp::write::h917062bce4ff48c3
                        at /build/rustc-1.14.0+dfsg1/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
2:     0x5650bd0b068f - std::panicking::default_hook::{{closure}}::h0bacac31b5ed1870
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:247
3:     0x5650bd0aee7c - std::panicking::default_hook::h5897799da33ece67
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:263
4:     0x5650bd0af4d7 - std::panicking::rust_panic_with_hook::h109e116a3a861224
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:451
5:     0x5650bd0af364 - std::panicking::begin_panic::hbb38be1379e09df0
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:413
6:     0x5650bd0af289 - std::panicking::begin_panic_fmt::h26713cea9bce3ab0
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:397
7:     0x5650bd0af217 - rust_begin_unwind
                        at /build/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:373
8:     0x5650bd0e2f3d - core::panicking::panic_fmt::hcfbb59eeb7f27f75
                        at /build/rustc-1.14.0+dfsg1/src/libcore/panicking.rs:69
9:     0x5650bd0a6e84 - core::result::unwrap_failed::h15a0fc826f4081f4
10:     0x5650bd0b7ffa - __rust_maybe_catch_panic
                        at /build/rustc-1.14.0+dfsg1/src/libpanic_unwind/lib.rs:97
11:     0x5650bd0a6fc1 - <F as alloc::boxed::FnBox<A>>::call_box::he32a93ebea7bc7ad
12:     0x5650bd0ae6c4 - std::sys::imp::thread::Thread::new::thread_start::ha102a6120fc52763
                        at /build/rustc-1.14.0+dfsg1/src/liballoc/boxed.rs:605
                        at /build/rustc-1.14.0+dfsg1/src/libstd/sys_common/thread.rs:21
                        at /build/rustc-1.14.0+dfsg1/src/libstd/sys/unix/thread.rs:84
13:     0x7fc2d0042423 - start_thread
14:     0x7fc2cfb6e9be - __clone
15:                0x0 - <unknown>
