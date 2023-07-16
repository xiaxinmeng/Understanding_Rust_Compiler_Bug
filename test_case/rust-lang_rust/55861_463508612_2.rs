
#0  0xb6c855a4 in __futex_syscall3 () from /system/lib/libc.so
#1  0xb6c7768c in __pthread_cond_timedwait_relative ()
   from /system/lib/libc.so
#2  0xb6c776ec in __pthread_cond_timedwait () from /system/lib/libc.so
#3  0xb6c7b726 in pthread_join () from /system/lib/libc.so
#4  0xb6d03810 in std::sys::unix::thread::Thread::join::h07b837055b9f571f ()
   from libstd-a3b039e1022c4e23.so
#5  0xb6e57ccc in _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::hd81a7f2f5464521c ()
#6  0xb6e5e944 in collectionstests::catch_panic::test_panic_hook::h9f08377776546cb3 ()
#7  0xb6dc7ca8 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h3970079b2a2e8325 () from libtest-0c5e281140db6463.so
#8  0xb6d3361c in __rust_maybe_catch_panic () from libstd-a3b039e1022c4e23.so
#9  0xb6dce6d0 in test::run_test::run_test_inner::h59a42c89d1b0ae7a ()
   from libtest-0c5e281140db6463.so
#10 0xb6dce368 in test::run_test::hd576cc177e253177 ()
   from libtest-0c5e281140db6463.so
#11 0xb6dcb9fc in test::run_tests_console::h8730cd4311bcbbcb ()
   from libtest-0c5e281140db6463.so
#12 0xb6dc8284 in test::test_main::h343b975aae7c6b90 ()
   from libtest-0c5e281140db6463.so
#13 0xb6dc848c in test::test_main_static::hfd242c8df31b861a ()
   from libtest-0c5e281140db6463.so
#14 0xb6e57424 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf8fbc2e6d4c74fb5 ()
#15 0xb6d29244 in std::panicking::try::do_call::h49216886d8653049 ()
   from libstd-a3b039e1022c4e23.so
#16 0xb6d3361c in __rust_maybe_catch_panic () from libstd-a3b039e1022c4e23.so
#17 0xb6d276d4 in std::panic::catch_unwind::h9c8f0a4ad65b2a9d ()
   from libstd-a3b039e1022c4e23.so
#18 0xb6cffa10 in std::rt::lang_start_internal::hc01076141fd50efb ()
   from libstd-a3b039e1022c4e23.so
#19 0xb6e75078 in main ()
