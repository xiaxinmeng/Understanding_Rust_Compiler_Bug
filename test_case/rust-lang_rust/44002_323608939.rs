
$ rustc -Z sanitizer=thread 1.rs

$ ./1
==================
WARNING: ThreadSanitizer: data race (pid=35728)
  Write of size 4 at 0x00010a2cf6c0 by thread T1:
    #0 _1::main::_$u7b$$u7b$closure$u7d$$u7d$::h2b2c1fdcf5ad676c <null>:204478040 (1:x86_64+0x100007758)
    #1 std::sys_common::backtrace::__rust_begin_short_backtrace::hf23e8bb2d4d2addb <null>:204478040 (1:x86_64+0x100001db5)
    #2 std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h1502a604e2389b0c <null>:204478040 (1:x86_64+0x100002ae5)
    #3 _$LT$std..panic..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hb06834c90f50dd84 <null>:204478040 (1:x86_64+0x100000c45)
    #4 std::panicking::try::do_call::h04f3fa6c6abda8d6 <null>:204478040 (1:x86_64+0x100002c99)
    #5 __rust_maybe_catch_panic <null>:204478040 (1:x86_64+0x1000315bc)
    #6 std::panic::catch_unwind::hf462c68ec7a63481 <null>:204478040 (1:x86_64+0x100001e67)
    #7 std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::hcddefde833003ffd <null>:204478040 (1:x86_64+0x1000028d9)
    #8 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h06c03c48d4910c94 <null>:204478040 (1:x86_64+0x100005cf4)
    #9 std::sys::imp::thread::Thread::new::thread_start::h431e210be94914a5 <null>:204478040 (1:x86_64+0x10000cf8b)

  Previous write of size 4 at 0x00010a2cf6c0 by main thread:
    #0 _1::main::hce80b422be4d604c <null>:204478040 (1:x86_64+0x10000769c)
    #1 __rust_maybe_catch_panic <null>:204478040 (1:x86_64+0x1000315bc)
    #2 start <null>:204479048 (libdyld.dylib:x86_64+0x5234)

  Location is global '_1::ANSWER::ha9074b9fd781a79e' at 0x00010a2cf6c0 (1+0x0001000476c0)

  Thread T1 (tid=8366680, running) created by main thread at:
    #0 pthread_create <null>:205529688 (libclang_rt.tsan_osx_dynamic.dylib:x86_64+0x7121)
    #1 std::sys::imp::thread::Thread::new::hc100d0d9e6164624 <null>:204478040 (1:x86_64+0x10000cc35)
    #2 std::thread::spawn::ha62bfd87b51b951c <null>:204478040 (1:x86_64+0x100001ee5)
    #3 _1::main::hce80b422be4d604c <null>:204478040 (1:x86_64+0x10000768c)
    #4 __rust_maybe_catch_panic <null>:204478040 (1:x86_64+0x1000315bc)
    #5 start <null>:204479048 (libdyld.dylib:x86_64+0x5234)

SUMMARY: ThreadSanitizer: data race (1:x86_64+0x100007758) in _1::main::_$u7b$$u7b$closure$u7d$$u7d$::h2b2c1fdcf5ad676c
==================
ThreadSanitizer: reported 1 warnings
Abort trap: 6

$ rustc -vV
rustc 1.21.0-nightly (7ac979d8c 2017-08-16)
binary: rustc
commit-hash: 7ac979d8cbe97241fd39f4037e1d4069caaff4d2
commit-date: 2017-08-16
host: x86_64-apple-darwin
release: 1.21.0-nightly
LLVM version: 4.0
