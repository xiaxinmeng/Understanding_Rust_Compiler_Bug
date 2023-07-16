 text
==42048== Memcheck, a memory error detector
==42048== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==42048== Using Valgrind-3.19.0.GIT-lbmacos and LibVEX; rerun with -h for copyright info
==42048== Command: ./std-b55978f9760549f9 --exact time::tests::instant_math
==42048== 
==42048== 
==42048== Support for macOS 11 and later is currently experimental
==42048== Some reports (especially memory leaks) might be missing or incorrect
...
chaos errors
...
running 1 test
==42048== Thread 2 time::tests::instant_math:
==42048== Conditional jump or move depends on uninitialised value(s)
==42048==    at 0x100D47624: <core::time::Duration as core::fmt::Debug>::fmt::fmt_decimal (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libstd-934770f7c15208a8.dylib)
==42048==    by 0x100D47115: <core::time::Duration as core::fmt::Debug>::fmt (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libstd-934770f7c15208a8.dylib)
==42048==    by 0x100D5E199: core::fmt::write (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libstd-934770f7c15208a8.dylib)
==42048==    by 0x100CBD534: std::io::stdio::_print (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libstd-934770f7c15208a8.dylib)
==42048==    by 0x1000F8A95: std::time::tests::instant_math (in ./std-b55978f9760549f9)
==42048==    by 0x100688C85: test::__rust_begin_short_backtrace (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libtest-be7efaea92be4066.dylib)
==42048==    by 0x100688DCB: test::run_test_in_process (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libtest-be7efaea92be4066.dylib)
==42048==    by 0x1006745B9: std::sys_common::backtrace::__rust_begin_short_backtrace (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libtest-be7efaea92be4066.dylib)
==42048==    by 0x10067C404: core::ops::function::FnOnce::call_once{{vtable.shim}} (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libtest-be7efaea92be4066.dylib)
==42048==    by 0x100CE2F06: std::sys::unix::thread::Thread::new::thread_start (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libstd-934770f7c15208a8.dylib)
==42048==    by 0x7FF80E0AC4E0: ??? (in /dev/ttys003)
==42048==    by 0x7FF80E0A7F6A: ??? (in /dev/ttys003)
==42048==  Uninitialised value was created by a stack allocation
==42048==    at 0x100D5DFF0: core::fmt::write (in /Users/m/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libstd-934770f7c15208a8.dylib)
==42048== 
...
some more errors
