
$ RUST_BACKTRACE=1 ./target/debug/ld-so-backtraces
thread 'main' panicked at 'explicit panic', src/main.rs:1:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/panicking.rs:143:14
   2: core::panicking::panic
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/panicking.rs:48:5
   3: ld_so_backtraces::main
             at ./src/main.rs:1:13
   4: core::ops::function::FnOnce::call_once
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
$ RUST_BACKTRACE=1 valgrind ./target/debug/ld-so-backtraces
==136503== Memcheck, a memory error detector
==136503== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==136503== Using Valgrind-3.15.0 and LibVEX; rerun with -h for copyright info
==136503== Command: ./target/debug/ld-so-backtraces
==136503==
thread 'main' panicked at 'explicit panic', src/main.rs:1:13
stack backtrace:
==136503== Syscall param statx(file_name) points to unaddressable byte(s)
==136503==    at 0x49A588E: statx (statx.c:29)
==136503==    by 0x1282E1: statx (weak.rs:178)
==136503==    by 0x1282E1: std::sys::unix::fs::try_statx (fs.rs:150)
==136503==    by 0x129555: file_attr (fs.rs:810)
==136503==    by 0x129555: metadata (fs.rs:494)
==136503==    by 0x129555: std::backtrace_rs::symbolize::gimli::mmap (gimli.rs:138)
==136503==    by 0x1298D7: new (elf.rs:21)
==136503==    by 0x1298D7: mapping_for_lib (gimli.rs:320)
==136503==    by 0x1298D7: std::backtrace_rs::symbolize::gimli::resolve::{{closure}} (gimli.rs:354)
==136503==    by 0x129794: with_global<std::backtrace_rs::symbolize::gimli::resolve::{closure_env#1}> (gimli.rs:266)
==136503==    by 0x129794: std::backtrace_rs::symbolize::gimli::resolve (gimli.rs:346)
==136503==    by 0x1230AE: resolve_frame_unsynchronized<std::sys_common::backtrace::_print_fmt::{closure#1}::{closure_env#0}> (mod.rs:178)
==136503==    by 0x1230AE: std::sys_common::backtrace::_print_fmt::{{closure}} (backtrace.rs:73)
==136503==    by 0x1283A0: call_mut<(&std::backtrace_rs::backtrace::Frame), dyn core::ops::function::FnMut<(&std::backtrace_rs::backtrace::Frame), Output=bool>> (function.rs:269)
==136503==    by 0x1283A0: std::backtrace_rs::backtrace::libunwind::trace::trace_fn (libunwind.rs:105)
==136503==    by 0x4865793: _Unwind_Backtrace (in /usr/lib/x86_64-linux-gnu/libgcc_s.so.1)
==136503==    by 0x122F1C: trace (libunwind.rs:93)
==136503==    by 0x122F1C: trace_unsynchronized<std::sys_common::backtrace::_print_fmt::{closure_env#1}> (mod.rs:66)
==136503==    by 0x122F1C: _print_fmt (backtrace.rs:66)
==136503==    by 0x122F1C: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt (backtrace.rs:45)
==136503==    by 0x13EC0B: core::fmt::write (mod.rs:1190)
==136503==    by 0x11FF77: std::io::Write::write_fmt (mod.rs:1657)
==136503==    by 0x124A73: _print (backtrace.rs:48)
==136503==    by 0x124A73: print (backtrace.rs:35)
==136503==    by 0x124A73: std::panicking::default_hook::{{closure}} (panicking.rs:292)
==136503==  Address 0x0 is not stack'd, malloc'd or (recently) free'd
==136503==
==136503== Syscall param statx(buf) points to unaddressable byte(s)
==136503==    at 0x49A588E: statx (statx.c:29)
==136503==    by 0x1282E1: statx (weak.rs:178)
==136503==    by 0x1282E1: std::sys::unix::fs::try_statx (fs.rs:150)
==136503==    by 0x129555: file_attr (fs.rs:810)
==136503==    by 0x129555: metadata (fs.rs:494)
==136503==    by 0x129555: std::backtrace_rs::symbolize::gimli::mmap (gimli.rs:138)
==136503==    by 0x1298D7: new (elf.rs:21)
==136503==    by 0x1298D7: mapping_for_lib (gimli.rs:320)
==136503==    by 0x1298D7: std::backtrace_rs::symbolize::gimli::resolve::{{closure}} (gimli.rs:354)
==136503==    by 0x129794: with_global<std::backtrace_rs::symbolize::gimli::resolve::{closure_env#1}> (gimli.rs:266)
==136503==    by 0x129794: std::backtrace_rs::symbolize::gimli::resolve (gimli.rs:346)
==136503==    by 0x1230AE: resolve_frame_unsynchronized<std::sys_common::backtrace::_print_fmt::{closure#1}::{closure_env#0}> (mod.rs:178)
==136503==    by 0x1230AE: std::sys_common::backtrace::_print_fmt::{{closure}} (backtrace.rs:73)
==136503==    by 0x1283A0: call_mut<(&std::backtrace_rs::backtrace::Frame), dyn core::ops::function::FnMut<(&std::backtrace_rs::backtrace::Frame), Output=bool>> (function.rs:269)
==136503==    by 0x1283A0: std::backtrace_rs::backtrace::libunwind::trace::trace_fn (libunwind.rs:105)
==136503==    by 0x4865793: _Unwind_Backtrace (in /usr/lib/x86_64-linux-gnu/libgcc_s.so.1)
==136503==    by 0x122F1C: trace (libunwind.rs:93)
==136503==    by 0x122F1C: trace_unsynchronized<std::sys_common::backtrace::_print_fmt::{closure_env#1}> (mod.rs:66)
==136503==    by 0x122F1C: _print_fmt (backtrace.rs:66)
==136503==    by 0x122F1C: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt (backtrace.rs:45)
==136503==    by 0x13EC0B: core::fmt::write (mod.rs:1190)
==136503==    by 0x11FF77: std::io::Write::write_fmt (mod.rs:1657)
==136503==    by 0x124A73: _print (backtrace.rs:48)
==136503==    by 0x124A73: print (backtrace.rs:35)
==136503==    by 0x124A73: std::panicking::default_hook::{{closure}} (panicking.rs:292)
==136503==  Address 0x0 is not stack'd, malloc'd or (recently) free'd
==136503==
   0: rust_begin_unwind
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/panicking.rs:143:14
   2: core::panicking::panic
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/panicking.rs:48:5
   3: ld_so_backtraces::main
             at ./src/main.rs:1:13
   4: core::ops::function::FnOnce::call_once
             at /rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
==136503==
==136503== HEAP SUMMARY:
==136503==     in use at exit: 1,636,120 bytes in 2,702 blocks
==136503==   total heap usage: 9,340 allocs, 6,638 frees, 5,711,794 bytes allocated
==136503==
==136503== LEAK SUMMARY:
==136503==    definitely lost: 0 bytes in 0 blocks
==136503==    indirectly lost: 0 bytes in 0 blocks
==136503==      possibly lost: 0 bytes in 0 blocks
==136503==    still reachable: 1,636,120 bytes in 2,702 blocks
==136503==         suppressed: 0 bytes in 0 blocks
==136503== Rerun with --leak-check=full to see details of leaked memory
==136503==
==136503== For lists of detected and suppressed errors, rerun with: -s
==136503== ERROR SUMMARY: 2 errors from 2 contexts (suppressed: 0 from 0)
