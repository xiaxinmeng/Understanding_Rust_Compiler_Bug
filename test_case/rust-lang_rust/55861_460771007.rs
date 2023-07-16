
Program terminated with signal SIGSEGV, Segmentation fault.

#0  0x08166877 in unbin (i=8, c=0xa048008) at src/malloc/malloc.c:195
#1  malloc (n=<optimized out>) at src/malloc/malloc.c:322
#2  0x0814ddd9 in alloc () at src/libstd/sys/unix/alloc.rs:11
#3  __rdl_alloc () at src/libstd/alloc.rs:233
#4  0x08127c2d in alloc () at /rustc/b6fdcffc3d9d0b28d3a1fc34c49221ff13617b43/src/liballoc/alloc.rs:72
#5  exchange_malloc () at /rustc/b6fdcffc3d9d0b28d3a1fc34c49221ff13617b43/src/liballoc/alloc.rs:182
#6  new<core::cell::UnsafeCell<core::option::Option<core::result::Result<(), alloc::boxed::Box<Any>>>>> () at /rustc/b6fdcffc3d9d0b28d3a1fc34c49221ff13617b43/src/liballoc/sync.rs:288
#7  spawn_unchecked<closure,()> () at /rustc/b6fdcffc3d9d0b28d3a1fc34c49221ff13617b43/src/libstd/thread/mod.rs:458
#8  spawn<closure,()> () at /rustc/b6fdcffc3d9d0b28d3a1fc34c49221ff13617b43/src/libstd/thread/mod.rs:382
#9  test::run_test::run_test_inner::hb0857806220adfa0 () at src/libtest/lib.rs:1450
#10 0x0812645e in test::run_test::h5d2da69c3af16b8e () at src/libtest/lib.rs:1471
#11 0x08120a84 in run_tests<closure> () at src/libtest/lib.rs:1161
#12 test::run_tests_console::hd61c1544d577a32b () at src/libtest/lib.rs:957
#13 0x08119522 in test::test_main::h2355f0b379f819ba () at src/libtest/lib.rs:290
#14 0x08119c07 in test::test_main_static::h90a75711843ac1f7 () at src/libtest/lib.rs:324
#15 0x08106785 in collectionstests::main::h89799a166ae8ba23 ()
