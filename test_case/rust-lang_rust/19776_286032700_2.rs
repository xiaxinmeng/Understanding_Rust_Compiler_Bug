
==18929== Memcheck, a memory error detector
==18929== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==18929== Using Valgrind-3.11.0 and LibVEX; rerun with -h for copyright info
==18929== Command: ./learn
==18929==
==18929== Conditional jump or move depends on uninitialised value(s)
==18929==    at 0x1174E1: std::io::stdio::stdout::stdout_init::h8c219ec6edaca9e2 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11556E: _$LT$std..io..lazy..Lazy$LT$T$GT$$GT$::get::h3818fe46065008a9 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x1179AE: std::io::stdio::_print::hb65c73db7e5ba8b4 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF2F: learn::main::h86b1088ba10af1af (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11CAC2: std::panicking::try::do_call::h4af13c61a0885795 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x121A30: __rust_maybe_catch_panic (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11CA77: std::panicking::try::h5d0504a555a21656 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x118CC8: std::panic::catch_unwind::h8daeb6c836b441af (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10CA36: std::rt::lang_start::h74f5ad66fb152c57 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF69: main (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==  Uninitialised value was created by a stack allocation
==18929==    at 0x117482: std::io::stdio::stdout::stdout_init::h8c219ec6edaca9e2 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==
How are you today
==18929==
==18929== HEAP SUMMARY:
==18929==     in use at exit: 277 bytes in 7 blocks
==18929==   total heap usage: 23 allocs, 16 frees, 3,560 bytes allocated
==18929==
==18929== 5 bytes in 1 blocks are still reachable in loss record 1 of 7
==18929==    at 0x4C2FD5F: realloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==18929==    by 0x10CB12: _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::reserve_exact::h4edf8f87d342a05f (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x118AE5: std::ffi::c_str::CString::from_vec_unchecked::hdcef4de01dead576 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x118A7D: std::ffi::c_str::CString::_new::hc7b7f6376e99eb50 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x1189B5: std::ffi::c_str::CString::new::h31a27631ea3d129f (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10E01B: std::thread::Thread::new::hdbde4e93369830a6 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10E38C: _$LT$std..thread..Thread$u20$as$u20$std..sys_common..thread_info..NewThread$GT$::new::hd92ec6def8113f36 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10CA08: std::rt::lang_start::h74f5ad66fb152c57 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF69: main (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==
==18929== 24 bytes in 1 blocks are still reachable in loss record 2 of 7
==18929==    at 0x4C2DB8F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==18929==    by 0x10C016: _$LT$std..thread..local..os..Key$LT$T$GT$$GT$::get::h4cb8adf76f34a8a8 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10EDAD: _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::h7268ed135fa874a3 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11CAE8: std::panicking::panicking::hf8bff814c3b72c57 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10DBEA: _$LT$std..sys_common..remutex..ReentrantMutex$LT$T$GT$$GT$::lock::hc072fd20cb222349 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x117671: _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::ha9551f3ca8d595e4 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x117A00: std::io::stdio::_print::hb65c73db7e5ba8b4 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF2F: learn::main::h86b1088ba10af1af (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11CAC2: std::panicking::try::do_call::h4af13c61a0885795 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x121A30: __rust_maybe_catch_panic (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11CA77: std::panicking::try::h5d0504a555a21656 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x118CC8: std::panic::catch_unwind::h8daeb6c836b441af (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==
==18929== 40 bytes in 1 blocks are still reachable in loss record 3 of 7
==18929==    at 0x4C2DB8F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==18929==    by 0x113E10: _$LT$std..sync..mutex..Mutex$LT$T$GT$$GT$::new::h2c7659a20efbd015 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10E086: std::thread::Thread::new::hdbde4e93369830a6 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10E38C: _$LT$std..thread..Thread$u20$as$u20$std..sys_common..thread_info..NewThread$GT$::new::hd92ec6def8113f36 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10CA08: std::rt::lang_start::h74f5ad66fb152c57 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF69: main (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==
==18929== 40 bytes in 1 blocks are still reachable in loss record 4 of 7
==18929==    at 0x4C2DB8F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==18929==    by 0x10C136: _$LT$std..thread..local..os..Key$LT$T$GT$$GT$::get::hf9087164760e6dbd (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10F132: _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::state::h0545415cf9387211 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x117986: std::io::stdio::_print::hb65c73db7e5ba8b4 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF2F: learn::main::h86b1088ba10af1af (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11CAC2: std::panicking::try::do_call::h4af13c61a0885795 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x121A30: __rust_maybe_catch_panic (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11CA77: std::panicking::try::h5d0504a555a21656 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x118CC8: std::panic::catch_unwind::h8daeb6c836b441af (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10CA36: std::rt::lang_start::h74f5ad66fb152c57 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF69: main (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==
==18929== 48 bytes in 1 blocks are still reachable in loss record 5 of 7
==18929==    at 0x4C2DB8F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==18929==    by 0x10D925: std::sync::condvar::Condvar::new::h9b5a8a20b184aadc (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10E090: std::thread::Thread::new::hdbde4e93369830a6 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10E38C: _$LT$std..thread..Thread$u20$as$u20$std..sys_common..thread_info..NewThread$GT$::new::hd92ec6def8113f36 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10CA08: std::rt::lang_start::h74f5ad66fb152c57 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF69: main (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==
==18929== 48 bytes in 1 blocks are still reachable in loss record 6 of 7
==18929==    at 0x4C2DB8F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==18929==    by 0x10C0A6: _$LT$std..thread..local..os..Key$LT$T$GT$$GT$::get::hbf071f63bdfa8db8 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10EE1B: _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::ha1974f44b6021a71 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x11BE2D: std::sys_common::thread_info::set::hcd1ca419433f28db (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10CA1F: std::rt::lang_start::h74f5ad66fb152c57 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF69: main (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==
==18929== 72 bytes in 1 blocks are still reachable in loss record 7 of 7
==18929==    at 0x4C2DB8F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==18929==    by 0x10E0C6: std::thread::Thread::new::hdbde4e93369830a6 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10E38C: _$LT$std..thread..Thread$u20$as$u20$std..sys_common..thread_info..NewThread$GT$::new::hd92ec6def8113f36 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10CA08: std::rt::lang_start::h74f5ad66fb152c57 (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==    by 0x10BF69: main (in /home/tschottdorf/rust-nojemalloc/learn)
==18929==
==18929== LEAK SUMMARY:
==18929==    definitely lost: 0 bytes in 0 blocks
==18929==    indirectly lost: 0 bytes in 0 blocks
==18929==      possibly lost: 0 bytes in 0 blocks
==18929==    still reachable: 277 bytes in 7 blocks
==18929==         suppressed: 0 bytes in 0 blocks
==18929==
==18929== For counts of detected and suppressed errors, rerun with: -v
==18929== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 0 from 0)
