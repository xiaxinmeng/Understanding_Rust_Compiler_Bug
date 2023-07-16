

---- [compile-fail] compile-fail/asm-out-read-uninit.rs stdout ----
	

executing powerpc64le-unknown-linux-gnu/stage2/bin/rustc /root/rustc-1.14.0+dfsg1/src/test/compile-fail/asm-out-read-uninit.rs -L powerpc64le-unknown-linux-gnu/test/compile-fail/ --target=powerpc64le-unknown-linux-gnu --error-format json -L powerpc64le-unknown-linux-gnu/test/compile-fail/asm-out-read-uninit.stage2-powerpc64le-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o powerpc64le-unknown-linux-gnu/test/compile-fail/asm-out-read-uninit.stage2-powerpc64le-unknown-linux-gnu -C link-args=-Wl,-z,relro -C rpath -O -L powerpc64le-unknown-linux-gnu/rt
------stdout------------------------------

------stderr------------------------------
{"message":"function is never used: `foo`, #[warn(dead_code)] on by default","code":null,"level":"warning","spans":[{"file_name":"/root/rustc-1.14.0+dfsg1/src/test/compile-fail/asm-out-read-uninit.rs","byte_start":507,"byte_end":546,"line_start":15,"line_end":15,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"fn foo(x: isize) { println!(\"{}\", x); }","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}

------------------------------------------

error: compile-fail test compiled successfully!
status: exit code: 0
command: powerpc64le-unknown-linux-gnu/stage2/bin/rustc /root/rustc-1.14.0+dfsg1/src/test/compile-fail/asm-out-read-uninit.rs -L powerpc64le-unknown-linux-gnu/test/compile-fail/ --target=powerpc64le-unknown-linux-gnu --error-format json -L powerpc64le-unknown-linux-gnu/test/compile-fail/asm-out-read-uninit.stage2-powerpc64le-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o powerpc64le-unknown-linux-gnu/test/compile-fail/asm-out-read-uninit.stage2-powerpc64le-unknown-linux-gnu -C link-args=-Wl,-z,relro -C rpath -O -L powerpc64le-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
{"message":"function is never used: `foo`, #[warn(dead_code)] on by default","code":null,"level":"warning","spans":[{"file_name":"/root/rustc-1.14.0+dfsg1/src/test/compile-fail/asm-out-read-uninit.rs","byte_start":507,"byte_end":546,"line_start":15,"line_end":15,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"fn foo(x: isize) { println!(\"{}\", x); }","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}

------------------------------------------

thread '[compile-fail] compile-fail/asm-out-read-uninit.rs' panicked at 'explicit panic', /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/runtest.rs:2394
stack backtrace:
   1:     0x3fffb6d33c13 - std::sys::imp::backtrace::tracing::imp::write::h917062bce4ff48c3
                        at /root/rustc-1.14.0+dfsg1/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x3fffb6d74a5f - std::panicking::default_hook::{{closure}}::h0bacac31b5ed1870
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:247
   3:     0x3fffb6d61dbb - std::panicking::default_hook::h5897799da33ece67
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:257
   4:     0x3fffb6d62a3f - std::panicking::rust_panic_with_hook::h109e116a3a861224
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:451
   5:         0x591ccbb3 - std::panicking::begin_panic::h634e2b37a96f78d4
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:413
   6:         0x5926776b - compiletest::runtest::ProcRes::fatal::h54a5408aa7f1b4ae
                        at /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/runtest.rs:2394
   7:         0x5925aea3 - compiletest::runtest::TestCx::fatal_proc_rec::h048763087fdecee5
                        at /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/runtest.rs:1610
   8:         0x59246167 - compiletest::runtest::TestCx::run_cfail_test::h6107dfa9bf737ccb
                        at /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/runtest.rs:147
   9:         0x59245dbf - compiletest::runtest::TestCx::run_revision::h28948cf4de3d8165
                        at /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/runtest.rs:113
  10:         0x59245a1b - compiletest::runtest::run::h9a013f2d7d043e98
                        at /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/runtest.rs:68
  11:         0x59279f37 - compiletest::make_test_closure::{{closure}}::h1d99800c1710baec
                        at /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/main.rs:475
  12:         0x591d722f - <F as test::FnBox<T>>::call_box::h4ad2e7c32cefe3ee
                        at /root/rustc-1.14.0+dfsg1/src/libtest/lib.rs:141
  13:     0x3fffb70c01d7 - test::run_test::run_test_inner::{{closure}}::{{closure}}::h45ac1c5b9ec21047
                        at /root/rustc-1.14.0+dfsg1/src/libtest/lib.rs:1211
  14:     0x3fffb70a4f53 - <std::panic::AssertUnwindSafe<F> as core::ops::FnOnce<()>>::call_once::h25100c493af17208
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panic.rs:295
  15:     0x3fffb705774f - std::panicking::try::do_call::h83b58d81c874206d
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:356
  16:     0x3fffb6d7e197 - __rust_try
  17:     0x3fffb6d7de73 - __rust_maybe_catch_panic
                        at /root/rustc-1.14.0+dfsg1/src/libpanic_unwind/lib.rs:97
  18:     0x3fffb7057227 - std::panicking::try::hc02417aadfb71d2a
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:332
  19:     0x3fffb705648b - std::panic::catch_unwind::h5a63093655e8b6d3
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panic.rs:351
  20:     0x3fffb70c04d7 - test::run_test::run_test_inner::{{closure}}::h6976ff3c0d36456b
                        at /root/rustc-1.14.0+dfsg1/src/libtest/lib.rs:1210
  21:     0x3fffb70a5003 - <std::panic::AssertUnwindSafe<F> as core::ops::FnOnce<()>>::call_once::h58d3803097181a5b
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panic.rs:295
  22:     0x3fffb70575c3 - std::panicking::try::do_call::h636b19c00d03e824
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:356
  23:     0x3fffb6d7e197 - __rust_try
  24:     0x3fffb6d7de73 - __rust_maybe_catch_panic
                        at /root/rustc-1.14.0+dfsg1/src/libpanic_unwind/lib.rs:97
  25:     0x3fffb7056ed7 - std::panicking::try::h903e0fe874149ae4
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:332
  26:     0x3fffb70563fb - std::panic::catch_unwind::h1d25dd81ac04550e
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panic.rs:351
  27:     0x3fffb70bea93 - std::thread::Builder::spawn::{{closure}}::h28a406b7a5ba6398
                        at /root/rustc-1.14.0+dfsg1/src/libstd/thread/mod.rs:287
  28:     0x3fffb708353f - <F as alloc::boxed::FnBox<A>>::call_box::h15256864a2eaf226
                        at /root/rustc-1.14.0+dfsg1/src/liballoc/boxed.rs:595
  29:     0x3fffb6c76f03 - <Box<alloc::boxed::FnBox<A, Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..FnOnce$LT$A$GT$$GT$::call_once::hea74365c035f0007
                        at /root/rustc-1.14.0+dfsg1/src/liballoc/boxed.rs:605
  30:     0x3fffb6d2e9b7 - std::sys_common::thread::start_thread::h92084fba4ec169b2
                        at /root/rustc-1.14.0+dfsg1/src/libstd/sys_common/thread.rs:21
  31:     0x3fffb6d5bdb7 - std::sys::imp::thread::Thread::new::thread_start::ha102a6120fc52763
                        at /root/rustc-1.14.0+dfsg1/src/libstd/sys/unix/thread.rs:84
  32:     0x3fffb691809b - <unknown>
