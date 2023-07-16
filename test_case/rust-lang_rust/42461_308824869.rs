
#0  0x000055ed856cc059 in BIG_fshl ()
#1  0x000055ed856ccb50 in BIG_mod ()
#2  0x000055ed856b36dd in milagro_crypto::big::{{impl}}::rmod (b=0x7ffacabfb308, c=0x55ed85757980 <CURVE_Order>) at src/big/mod.rs:220
#3  0x000055ed856b3b51 in milagro_crypto::big::{{impl}}::randomnum (q=0x55ed85757980 <CURVE_Order>, rng=0x7ffacabfb460) at src/big/mod.rs:322
#4  0x000055ed856b48b7 in milagro_crypto::big::tests::test_random_mod_order () at src/big/mod.rs:402
#5  0x000055ed856e5f02 in test::run_test::{{closure}} () at /checkout/src/libtest/lib.rs:1451
#6  core::ops::FnOnce::call_once<closure,(())> () at /checkout/src/libcore/ops.rs:2683
#7  test::{{impl}}::call_box<(),closure> () at /checkout/src/libtest/lib.rs:140
#8  0x000055ed857219fb in panic_unwind::__rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:98
#9  0x000055ed856da3d4 in std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> () at /checkout/src/libstd/panicking.rs:433
#10 std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> () at /checkout/src/libstd/panic.rs:361
#11 test::run_test::run_test_inner::{{closure}} () at /checkout/src/libtest/lib.rs:1390
#12 std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()> () at /checkout/src/libstd/sys_common/backtrace.rs:136
#13 0x000055ed856db173 in std::thread::{{impl}}::spawn::{{closure}}::{{closure}}<closure,()> () at /checkout/src/libstd/thread/mod.rs:364
#14 std::panic::{{impl}}::call_once<(),closure> () at /checkout/src/libstd/panic.rs:296
#15 std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()> () at /checkout/src/libstd/panicking.rs:454
#16 0x000055ed857219fb in panic_unwind::__rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:98
#17 0x000055ed856e10fd in std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> () at /checkout/src/libstd/panicking.rs:433
#18 std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> () at /checkout/src/libstd/panic.rs:361
#19 std::thread::{{impl}}::spawn::{{closure}}<closure,()> () at /checkout/src/libstd/thread/mod.rs:363
#20 alloc::boxed::{{impl}}::call_box<(),closure> () at /checkout/src/liballoc/boxed.rs:648
#21 0x000055ed85719826 in alloc::boxed::{{impl}}::call_once<(),()> () at /checkout/src/liballoc/boxed.rs:658
#22 std::sys_common::thread::start_thread () at /checkout/src/libstd/sys_common/thread.rs:21
#23 std::sys::imp::thread::{{impl}}::new::thread_start () at /checkout/src/libstd/sys/unix/thread.rs:84
#24 0x00007ffacc09e6ba in start_thread (arg=0x7ffacabfc700) at pthread_create.c:333
#25 0x00007ffacbbbe82d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
