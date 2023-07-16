
#0  malloc_usable_size (ptr=<optimized out>) at /checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/arena.h:743
#1  0x00005614c6ee2bdd in heapsize::heap_size_of_impl (ptr=0x4) at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/heapsize-0.3.9/src/lib.rs:50
#2  0x00005614c6ee2ba4 in heapsize::heap_size_of (ptr=0x4) at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/heapsize-0.3.9/src/lib.rs:36
#3  0x00005614c6ebd0e4 in heapsize::{{impl}}::heap_size_of_children<f32> (self=0x7f2bae3fe4d0) at /root/.cargo/registry/src/github.com-1ecc6299db9ec823/heapsize-0.3.9/src/lib.rs:234
#4  0x00005614c6ec30fd in euclid::length::{{impl}}::heap_size_of_children<euclid::length::tests::Inch,collections::vec::Vec<f32>> (self=0x7f2bae3fe4d0) at src/length.rs:49
#5  0x00005614c6ec4bd4 in euclid::length::tests::test_heapsizeof_length_vector () at src/length.rs:244
#6  0x00005614c6ef81dc in test::run_test::{{closure}} () at /checkout/src/libtest/lib.rs:1430
#7  core::ops::FnOnce::call_once<closure,(())> () at /checkout/src/libcore/ops.rs:2626
#8  test::{{impl}}::call_box<(),closure> () at /checkout/src/libtest/lib.rs:140
#9  0x00005614c6f3965b in panic_unwind::__rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:98
#10 0x00005614c6eea684 in std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> () at /checkout/src/libstd/panicking.rs:433
#11 std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> () at /checkout/src/libstd/panic.rs:361
#12 test::run_test::run_test_inner::{{closure}} () at /checkout/src/libtest/lib.rs:1375
#13 std::panic::{{impl}}::call_once<(),closure> () at /checkout/src/libstd/panic.rs:296
#14 std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()> () at /checkout/src/libstd/panicking.rs:454
#15 0x00005614c6f3965b in panic_unwind::__rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:98
#16 0x00005614c6ef258d in std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> () at /checkout/src/libstd/panicking.rs:433
#17 std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> () at /checkout/src/libstd/panic.rs:361
#18 std::thread::{{impl}}::spawn::{{closure}}<closure,()> () at /checkout/src/libstd/thread/mod.rs:361
#19 alloc::boxed::{{impl}}::call_box<(),closure> () at /checkout/src/liballoc/boxed.rs:648
#20 0x00005614c6f31386 in alloc::boxed::{{impl}}::call_once<(),()> () at /checkout/src/liballoc/boxed.rs:658
#21 std::sys_common::thread::start_thread () at /checkout/src/libstd/sys_common/thread.rs:21
#22 std::sys::imp::thread::{{impl}}::new::thread_start () at /checkout/src/libstd/sys/unix/thread.rs:84
#23 0x00007f2bb148d6ba in start_thread (arg=0x7f2bae3ff700) at pthread_create.c:333
#24 0x00007f2bb0fad82d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
