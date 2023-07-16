
failures:

---- [run-pass] run-pass/coerce-match.rs stdout ----

error: test run failed!
status: signal: 11
command: /usr/bin/valgrind x86_64-unknown-linux-gnu/test/run-pass/coerce-match.stage2-x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
==22243== Memcheck, a memory error detector
==22243== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==22243== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==22243== Command: x86_64-unknown-linux-gnu/test/run-pass/coerce-match.stage2-x86_64-unknown-linux-gnu
==22243== 
==22243== Warning: client switching stacks?  SP change: 0xffeffee60 --> 0xffedf63d8
==22243==          to suppress, use: --max-stackframe=2132616 or greater
==22243== Syscall param read(buf) points to unaddressable byte(s)
==22243==    at 0x40194F7: read (syscall-template.S:81)
==22243==    by 0x4005E0C: open_verify.constprop.6 (dl-load.c:2099)
==22243==    by 0x40061CE: open_path (dl-load.c:2216)
==22243==    by 0x4008F19: _dl_map_object (dl-load.c:2447)
==22243==    by 0x400D601: openaux (dl-deps.c:63)
==22243==    by 0x400FFF3: _dl_catch_error (dl-error.c:187)
==22243==    by 0x400DD04: _dl_map_object_deps (dl-deps.c:254)
==22243==    by 0x400315C: dl_main (rtld.c:1742)
==22243==    by 0x4017564: _dl_sysdep_start (dl-sysdep.c:249)
==22243==    by 0x4004CF7: _dl_start (rtld.c:332)
==22243==    by 0x40012D7: ??? (in /lib/x86_64-linux-gnu/ld-2.19.so)
==22243==  Address 0xffedf63f0 is on thread 1's stack
==22243== 
==22243== Warning: client switching stacks?  SP change: 0xffedf63e0 --> 0xffeffee88
==22243==          to suppress, use: --max-stackframe=2132648 or greater
==22243== Invalid free() / delete / delete[] / realloc()
==22243==    at 0x4F779D8: je_valgrind_freelike_block (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4FC1D28: rust_try_inner (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4FC1D15: rust_try (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F5DBC9: rt::lang_start::he0ccf12406aa1099d1z (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x5489EC4: (below main) (libc-start.c:287)
==22243==  Address 0x6c26000 is 0 bytes inside a block of size 32 free'd
==22243==    at 0x4F779D8: je_valgrind_freelike_block (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F5D5B2: rt::args::imp::put::hbd0ed9ed6213e26c8Rz (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F5D024: rt::args::init::hcc36baa9a67ccf9dlPz (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F5DB52: rt::lang_start::he0ccf12406aa1099d1z (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x5489EC4: (below main) (libc-start.c:287)
==22243== 
==22243== Invalid read of size 8
==22243==    at 0x4F92559: je_arena_dalloc_small (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F76FF1: je_sdallocx (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F5D1A2: rt::args::cleanup::hec9e413559adae2cDPz (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F5DC9C: rt::lang_start::he0ccf12406aa1099d1z (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x5489EC4: (below main) (libc-start.c:287)
==22243==  Address 0x6be92b8 is not stack'd, malloc'd or (recently) free'd
==22243== 
==22243== Invalid read of size 4
==22243==    at 0x5A3C414: pthread_mutex_lock (pthread_mutex_lock.c:66)
==22243==    by 0x4F92565: je_arena_dalloc_small (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F76FF1: je_sdallocx (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F5D1A2: rt::args::cleanup::hec9e413559adae2cDPz (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x4F5DC9C: rt::lang_start::he0ccf12406aa1099d1z (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==22243==    by 0x5489EC4: (below main) (libc-start.c:287)
==22243==  Address 0x10 is not stack'd, malloc'd or (recently) free'd
==22243== 
==21446== 
==21446== Process terminating with default action of signal 11 (SIGSEGV)
==21446==    at 0x549EBB9: raise (raise.c:56)
==21446==    by 0x4F4A704: sys::stack_overflow::imp::signal_handler::h7e9a49daa22611feMkw (in /home/chris/workspace/rust/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so)
==21446==    by 0x549EC2F: ??? (in /lib/x86_64-linux-gnu/libc-2.19.so)
==21446==    by 0x5A3C413: pthread_mutex_lock (pthread_mutex_lock.c:63)
==22243== 
==22243== HEAP SUMMARY:
==22243==     in use at exit: 264 bytes in 6 blocks
==22243==   total heap usage: 18 allocs, 13 frees, 1,672 bytes allocated
==22243== 
==22243== LEAK SUMMARY:
==22243==    definitely lost: 0 bytes in 0 blocks
==22243==    indirectly lost: 0 bytes in 0 blocks
==22243==      possibly lost: 0 bytes in 0 blocks
==22243==    still reachable: 264 bytes in 6 blocks
==22243==         suppressed: 0 bytes in 0 blocks
==22243== Rerun with --leak-check=full to see details of leaked memory
==22243== 
==22243== For counts of detected and suppressed errors, rerun with: -v
==22243== ERROR SUMMARY: 4 errors from 4 contexts (suppressed: 2 from 2)

------------------------------------------

thread '[run-pass] run-pass/coerce-match.rs' panicked at 'explicit panic', /home/chris/workspace/rust/src/compiletest/runtest.rs:1487



failures:
    [run-pass] run-pass/coerce-match.rs

test result: FAILED. 1756 passed; 1 failed; 27 ignored; 0 measured

