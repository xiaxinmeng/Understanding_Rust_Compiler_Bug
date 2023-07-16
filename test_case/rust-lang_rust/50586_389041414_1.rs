
$ valgrind target/release/examples/repro
==80783== Memcheck, a memory error detector
==80783== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==80783== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
==80783== Command: target/release/examples/repro
==80783== 
--80783-- run: /usr/bin/dsymutil "target/release/examples/repro"
==80783== 
==80783== Process terminating with default action of signal 11 (SIGSEGV)
==80783==  General Protection Fault
==80783==    at 0x100002CAB: thread_local::thread_id::get (in target/release/examples/repro)
==80783==    by 0x1000020DB: repro::main (in target/release/examples/repro)
==80783==    by 0x100001CE5: std::rt::lang_start::{{closure}} (in target/release/examples/repro)
==80783==    by 0x10000B227: _ZN3std9panicking3try7do_call17hc7b9e6190a1d9f3eE.llvm.7628423903506348692 (rt.rs:59)
==80783==    by 0x10001BFAE: __rust_maybe_catch_panic (lib.rs:102)
==80783==    by 0x100009D51: std::rt::lang_start_internal (panicking.rs:285)
==80783==    by 0x10000222B: main (in target/release/examples/repro)
==80783== 
==80783== HEAP SUMMARY:
==80783==     in use at exit: 22,839 bytes in 193 blocks
==80783==   total heap usage: 259 allocs, 66 frees, 28,607 bytes allocated
==80783== 
==80783== LEAK SUMMARY:
==80783==    definitely lost: 0 bytes in 0 blocks
==80783==    indirectly lost: 0 bytes in 0 blocks
==80783==      possibly lost: 2,064 bytes in 1 blocks
==80783==    still reachable: 184 bytes in 2 blocks
==80783==         suppressed: 20,591 bytes in 190 blocks
==80783== Rerun with --leak-check=full to see details of leaked memory
==80783== 
==80783== For counts of detected and suppressed errors, rerun with: -v
==80783== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
Segmentation fault: 11
