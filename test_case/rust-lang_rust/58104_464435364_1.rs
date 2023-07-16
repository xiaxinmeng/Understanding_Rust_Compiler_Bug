
#0  0x0000000000414e4f in futures_executor::thread_pool::ThreadPoolBuilder::create (self=0x7fffffffc170)
    at /home/nagisa/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-executor-0.2.1/src/thread_pool.rs:262
#1  0x0000000000413d6f in futures_executor::thread_pool::ThreadPool::new ()
    at /home/nagisa/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-executor-0.2.1/src/thread_pool.rs:78
#2  0x000000000040424d in tarpotest::b () at src/lib.rs:8
#3  0x0000000000404a4a in tarpotest::b::{{closure}} () at src/lib.rs:7
#4  0x00000000004047ae in core::ops::function::FnOnce::call_once ()
    at /rustc/7164a9f151a56316a382d8bc2b15ccf373e129ca/src/libcore/ops/function.rs:231
#5  0x000000000046105f in test::run_test::{{closure}} () at src/libtest/lib.rs:1475
#6  core::ops::function::FnOnce::call_once ()
    at /rustc/7164a9f151a56316a382d8bc2b15ccf373e129ca/src/libcore/ops/function.rs:231
#7  <F as alloc::boxed::FnBox<A>>::call_box ()
    at /rustc/7164a9f151a56316a382d8bc2b15ccf373e129ca/src/liballoc/boxed.rs:734
#8  0x00000000004cdbfa in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#9  0x000000000047f188 in std::panicking::try ()
    at /rustc/7164a9f151a56316a382d8bc2b15ccf373e129ca/src/libstd/panicking.rs:276
#10 std::panic::catch_unwind () at /rustc/7164a9f151a56316a382d8bc2b15ccf373e129ca/src/libstd/panic.rs:388
#11 test::run_test::run_test_inner::{{closure}} () at src/libtest/lib.rs:1430
#12 0x000000000047eaad in test::run_test::run_test_inner () at src/libtest/lib.rs:1452
#13 0x000000000047d22c in test::run_test () at src/libtest/lib.rs:1471
#14 0x0000000000475c8d in test::run_tests () at src/libtest/lib.rs:1150
#15 test::run_tests_console () at src/libtest/lib.rs:957
#16 0x000000000046d764 in test::test_main () at src/libtest/lib.rs:290
#17 0x000000000046dfb2 in test::test_main_static () at src/libtest/lib.rs:324
#18 0x0000000000404276 in tarpotest::main ()
#19 0x0000000000404ad0 in std::rt::lang_start::{{closure}} ()
    at /rustc/7164a9f151a56316a382d8bc2b15ccf373e129ca/src/libstd/rt.rs:64
#20 0x00000000004bbfb3 in std::rt::lang_start_internal::{{closure}} () at src/libstd/rt.rs:49
#21 std::panicking::try::do_call () at src/libstd/panicking.rs:297
#22 0x00000000004cdbfa in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#23 0x00000000004bcbc6 in std::panicking::try () at src/libstd/panicking.rs:276
#24 std::panic::catch_unwind () at src/libstd/panic.rs:388
#25 std::rt::lang_start_internal () at src/libstd/rt.rs:48
#26 0x0000000000404aa9 in std::rt::lang_start (main=0x404260 <tarpotest::main>, argc=2, argv=0x7fffffffdc08)
    at /rustc/7164a9f151a56316a382d8bc2b15ccf373e129ca/src/libstd/rt.rs:64
#27 0x00000000004042ad in main ()
