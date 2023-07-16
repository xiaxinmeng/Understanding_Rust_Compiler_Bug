
$ RUST_BACKTRACE=1 cargo test
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/action-80d4ecacee9f9bcc

running 4 tests
test license ... ok
test parser ... ok
test tostr ... ok
test errors ... FAILED

failures:

---- errors stdout ----
	thread 'errors' panicked at 'Action not invalid: depend type=require', /builds/rsdev/tests/utils/mod.rs:105
stack backtrace:
   1:     0x560d0dbebb5a - std::sys::imp::backtrace::tracing::imp::write::h917062bce4ff48c3
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x560d0dbf066f - std::panicking::default_hook::{{closure}}::h0bacac31b5ed1870
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:247
   3:     0x560d0dbee8a5 - std::panicking::default_hook::h5897799da33ece67
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:257
   4:     0x560d0dbeefb7 - std::panicking::rust_panic_with_hook::h109e116a3a861224
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:451
   5:     0x560d0dbeee44 - std::panicking::begin_panic::hbb38be1379e09df0
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:413
   6:     0x560d0dbeed69 - std::panicking::begin_panic_fmt::h26713cea9bce3ab0
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:397

This wasn't as helpful as I hoped by defa
   7:     0x560d0db4550b - action::utils::assert_invalid::h8e7640be05700633
                        at /builds/rsdev/tests/utils/mod.rs:105
   8:     0x560d0db4cb86 - action::errors::h54693ccbd5897fb6
                        at /builds/rsdev/tests/action.rs:389
   9:     0x560d0db5c3fe - <F as test::FnBox<T>>::call_box::he8581d59e8028413
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libtest/lib.rs:1265
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libtest/lib.rs:141
  10:     0x560d0dbf814a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libpanic_unwind/lib.rs:97
  11:     0x560d0db514ef - std::panicking::try::do_call::h636b19c00d03e824
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:332
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panic.rs:351
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libtest/lib.rs:1210
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panic.rs:295
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:356
  12:     0x560d0dbf814a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libpanic_unwind/lib.rs:97
  13:     0x560d0db57812 - <F as alloc::boxed::FnBox<A>>::call_box::h15256864a2eaf226
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:332
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/panic.rs:351
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/thread/mod.rs:287
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:595
  14:     0x560d0dbee014 - std::sys::imp::thread::Thread::new::thread_start::ha102a6120fc52763
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:605
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/obj/../src/libstd/sys/unix/thread.rs:84
  15:     0x7fa3b5c126b9 - start_thread
  16:     0x7fa3b573282c - clone
  17:                0x0 - <unknown>
