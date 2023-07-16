console
% env RUSTFLAGS="-Zsanitizer=memory -Cllvm-args=-msan-track-origins=2" cargo -Zbuild-std run
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `/home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check`
zsh: segmentation fault  env RUSTFLAGS="-Zsanitizer=memory -Cllvm-args=-msan-track-origins=2" cargo
% env RUSTFLAGS="-Zsanitizer=memory -Cllvm-args=-msan-track-origins=2" cargo -Zbuild-std r
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check`
Uninitialized bytes in __interceptor_memchr at offset 0 inside [0x701000000000, 4)
==31792==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x55eb5be5430e in std::sys::unix::memchr::memchr::h093351bf413e1811 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/sys/unix/memchr.rs:6:8
    #1 0x55eb5be5430e in std::memchr::memchr::h611c7bcd6977d5c8 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/memchr.rs:25:4
    #2 0x55eb5be5430e in std::ffi::c_str::CString::_new::hae9b6987fec5f7fd /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/ffi/c_str.rs:354:14
    #3 0x55eb5be5430e in std::ffi::c_str::CString::new::h1ea624224271aaf7 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/ffi/c_str.rs:350:8
    #4 0x55eb5be5430e in std::thread::Thread::new::_$u7b$$u7b$closure$u7d$$u7d$::h1c0ee72b60a277c3 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/thread/mod.rs:1139:25
    #5 0x55eb5be5430e in core::option::Option$LT$T$GT$::map::hcc879affbaa7fe1d /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libcore/option.rs:450:28
    #6 0x55eb5be5430e in std::thread::Thread::new::h50a07a669fcb2430 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/thread/mod.rs:1139:12
    #7 0x55eb5be5abf8 in std::rt::lang_start_internal::h9d8db41cd41d5c82 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/rt.rs:44:21
    #8 0x55eb5be511aa in std::rt::lang_start::h49972300d46b5330 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/rt.rs:67:4
    #9 0x55eb5be523ec in main (/home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check+0x913ec)
    #10 0x7fcb662fe09a in __libc_start_main (/lib/x86_64-linux-gnu/libc.so.6+0x2409a)
    #11 0x55eb5bdf5029 in _start (/home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check+0x34029)

  Uninitialized value was created by a heap allocation
    #0 0x55eb5bdfff9d in malloc /checkout/src/llvm-project/compiler-rt/lib/msan/msan_interceptors.cc:916:3
    #1 0x55eb5be5abbb in alloc::alloc::alloc::hcdcb9b2beecd2386 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/alloc.rs:81:4
    #2 0x55eb5be5abbb in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Alloc$GT$::alloc::h6874ec33458db448 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/alloc.rs:169:21
    #3 0x55eb5be5abbb in alloc::raw_vec::RawVec$LT$T$C$A$GT$::allocate_in::h6551de8396f43b6a /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/raw_vec.rs:88:73
    #4 0x55eb5be5abbb in alloc::raw_vec::RawVec$LT$T$GT$::with_capacity::ha9f3e23fbbb76ceb /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/raw_vec.rs:140:8
    #5 0x55eb5be5abbb in alloc::vec::Vec$LT$T$GT$::with_capacity::h07a6336c6cdafeb1 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/vec.rs:355:19
    #6 0x55eb5be5abbb in alloc::slice::hack::to_vec::h9cacb4bd68db4f4e /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/slice.rs:158:25
    #7 0x55eb5be5abbb in alloc::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::to_vec::hb97d0cd8f5bb7f3b /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/slice.rs:394:8
    #8 0x55eb5be5abbb in alloc::slice::_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$::to_owned::h2141d6531caed0bc /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/slice.rs:727:8
    #9 0x55eb5be5abbb in alloc::str::_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$::to_owned::h8c20d4eb37699c47 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/liballoc/str.rs:205:45
    #10 0x55eb5be5abbb in std::rt::lang_start_internal::h9d8db41cd41d5c82 /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/rt.rs:44:38

SUMMARY: MemorySanitizer: use-of-uninitialized-value /rustc/6d3f4e0aab3e36ceb8b83d1e9467514685f6b751/src/libstd/sys/unix/memchr.rs:6:8 in std::sys::unix::memchr::memchr::h093351bf413e1811
Exiting
