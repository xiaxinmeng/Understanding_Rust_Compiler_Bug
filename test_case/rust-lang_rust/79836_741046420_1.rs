shell
RUSTFLAGS="-Z sanitizer=thread" cargo +nightly test --target x86_64-unknown-linux-gnu -j 8 --lib
   Compiling temp v0.1.0 (/mnt/d/r/code/FUN/AoC/temp)
    Finished test [unoptimized + debuginfo] target(s) in 3.55s
     Running target/x86_64-unknown-linux-gnu/debug/deps/temp-095a78e43e82b177

running 1 test
test tests::it_works ... ok
==================
WARNING: ThreadSanitizer: data race (pid=2378)
  Write of size 8 at 0x7b4000008000 by main thread:
    #0 free /rustc/llvm/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:708:3 (temp-095a78e43e82b177+0x1bcc8)
    #1 alloc::alloc::dealloc::h49222860122f83cb /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/alloc.rs:104:14 (temp-095a78e43e82b177+0x907ef)
    #2 _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::deallocate::hab070f75e853a0f9 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/alloc.rs:239:22
(temp-095a78e43e82b177+0x907ef)
    #3 alloc::alloc::box_free::hb71a745404480769 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/alloc.rs:334:9 (temp-095a78e43e82b177+0x907ef)
    #4 core::ptr::drop_in_place::hc04477b996b712f4 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/core/src/ptr/mod.rs:179:1 (temp-095a78e43e82b177+0x907ef)
    #5 _$LT$std..sync..mpsc..mpsc_queue..Queue$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::h91d8518b384ead20 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sy
nc/mpsc/mpsc_queue.rs:112:57 (temp-095a78e43e82b177+0x907ef)
    #6 core::ptr::drop_in_place::h9650e8c8039b88dc /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/core/src/ptr/mod.rs:179:1 (temp-095a78e43e82b177+0x907ef)
    #7 core::ptr::drop_in_place::h51cb6d3d33fea70a /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/core/src/ptr/mod.rs:179:1 (temp-095a78e43e82b177+0x907ef)
    #8 core::ops::function::FnOnce::call_once::h5a9930e2447997e9 /home/ubuntu/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:
227:5 (temp-095a78e43e82b177+0x89ab1)
    #9 std::sys_common::backtrace::__rust_begin_short_backtrace::h24f3df1126ff3b49 /home/ubuntu/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src
/sys_common/backtrace.rs:125:18 (temp-095a78e43e82b177+0x89677)
    #10 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha0a8f32cf2554108 /home/ubuntu/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs
:66:18 (temp-095a78e43e82b177+0x890c4)
    #11 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h54c39b9b8451875a /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845
fc/library/core/src/ops/function.rs:259:13 (temp-095a78e43e82b177+0xe40a6)
    #12 std::panicking::try::do_call::hcdfba51a06e21b42 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:379:40 (temp-095a78e43e82b177+0xe40a6)
    #13 std::panicking::try::hb36953ccd24f930d /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:343:19 (temp-095a78e43e82b177+0xe40a6)
    #14 std::panic::catch_unwind::hbb374cd03beb3824 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panic.rs:396:14 (temp-095a78e43e82b177+0xe40a6)
    #15 std::rt::lang_start_internal::h97dca9624db2aa85 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/rt.rs:51:25 (temp-095a78e43e82b177+0xe40a6)
    #16 main <null> (temp-095a78e43e82b177+0x8978a)

  Previous write of size 8 at 0x7b4000008000 by thread T1:
    #0 malloc /rustc/llvm/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:652:5 (temp-095a78e43e82b177+0x1b674)
    #1 alloc::alloc::alloc::h42bff26f59c33bd6 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/alloc.rs:86:14 (temp-095a78e43e82b177+0x8bc3f)
    #2 alloc::alloc::Global::alloc_impl::h6e8f952ef784c80f /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/alloc.rs:166:73 (temp-095a78e43e82b177+0x8bc3f)
    #3 _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::allocate::hcd5aa98f8fdb82d8 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/alloc.rs:226:9 (te
mp-095a78e43e82b177+0x8bc3f)
    #4 alloc::alloc::exchange_malloc::h8373e57f5f7edca9 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/alloc.rs:316:11 (temp-095a78e43e82b177+0x8bc3f)
    #5 std::sync::mpsc::mpsc_queue::Node$LT$T$GT$::new::h791a770dd4b02fed /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sync/mpsc/mpsc_queue.rs:56:23 (temp-095a78e43e
82b177+0x8bc3f)
    #6 std::sync::mpsc::mpsc_queue::Queue$LT$T$GT$::push::h97feedbf53291432 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sync/mpsc/mpsc_queue.rs:71:21 (temp-095a78e4
3e82b177+0x8bc3f)
    #7 std::sync::mpsc::shared::Packet$LT$T$GT$::send::h977d42fd693a300b /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sync/mpsc/shared.rs:166:9 (temp-095a78e43e82b17
7+0x8bc3f)
    #8 std::sync::mpsc::Sender$LT$T$GT$::send::h000bc7afee4ae066 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sync/mpsc/mod.rs:794:45 (temp-095a78e43e82b177+0x8bc3f)

  Thread T1 'tests::it_works' (tid=2380, finished) created by main thread at:
    #0 pthread_create /rustc/llvm/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:966:3 (temp-095a78e43e82b177+0x1cc2b)
    #1 std::sys::unix::thread::Thread::new::hd2dd4e6b2bfa56e2 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/sys/unix/thread.rs:50:19 (temp-095a78e43e82b177+0xe8f91)
    #2 core::ops::function::FnOnce::call_once::h5a9930e2447997e9 /home/ubuntu/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:
227:5 (temp-095a78e43e82b177+0x89ab1)
    #3 std::sys_common::backtrace::__rust_begin_short_backtrace::h24f3df1126ff3b49 /home/ubuntu/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src
/sys_common/backtrace.rs:125:18 (temp-095a78e43e82b177+0x89677)
    #4 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha0a8f32cf2554108 /home/ubuntu/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:
66:18 (temp-095a78e43e82b177+0x890c4)
    #5 core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h54c39b9b8451875a /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845f
c/library/core/src/ops/function.rs:259:13 (temp-095a78e43e82b177+0xe40a6)
    #6 std::panicking::try::do_call::hcdfba51a06e21b42 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:379:40 (temp-095a78e43e82b177+0xe40a6)
    #7 std::panicking::try::hb36953ccd24f930d /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:343:19 (temp-095a78e43e82b177+0xe40a6)
    #8 std::panic::catch_unwind::hbb374cd03beb3824 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panic.rs:396:14 (temp-095a78e43e82b177+0xe40a6)
    #9 std::rt::lang_start_internal::h97dca9624db2aa85 /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/rt.rs:51:25 (temp-095a78e43e82b177+0xe40a6)
    #10 main <null> (temp-095a78e43e82b177+0x8978a)

SUMMARY: ThreadSanitizer: data race /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/alloc/src/alloc.rs:104:14 in alloc::alloc::dealloc::h49222860122f83cb
==================

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s

ThreadSanitizer: reported 1 warnings
error: test failed, to rerun pass '--lib'
