
$ gdb -args rustc -v
GNU gdb (GDB) 7.12.1
(gdb) set environment LIBUNWIND_PRINT_UNWINDING=1 LIBUNWIND_PRINT_APIS=1
(gdb) run
Starting program: /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/bin/rustc -v
error: no input filename given

libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): pc=0x1f1ccb1d9ce4, start_ip=0x1f1ccb1d9c70, func=__rust_start_panic, lsda=0x0, personality=0x0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): pc=0x1f1ccb1cd529, start_ip=0x1f1ccb1cd520, func=.anonymous., lsda=0x0, personality=0x0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): pc=0x1f1ccb1cd4e6, start_ip=0x1f1ccb1cd330, func=_ZN3std9panicking20rust_panic_with_hook17hf2d11de5f905222bE, lsda=0x1f1ccb382c3c, personality=0x1f1ccb1d97a0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): calling personality function 0x1f1ccb1d97a0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): pc=0x1f1d7cc32d78, start_ip=0x1f1d7cc32d60, func=.anonymous., lsda=0x0, personality=0x0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): pc=0x1f1d7ce2953d, start_ip=0x1f1d7ce29350, func=_ZN5rustc7session11early_error17h0187f9fdb6331d60E, lsda=0x1f1d7d156c98, personality=0x1f1ccb1d97a0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): calling personality function 0x1f1ccb1d97a0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): pc=0x1f1d108ebe6d, start_ip=0x1f1d108e9a10, func=_ZN93_$LT$rustc_driver..RustcDefaultCalls$u20$as$u20$rustc_driver..CompilerCalls$LT$$u27$a$GT$$GT$8no_input17hc57ca5476db31dacE, lsda=0x1f1d10a7ad9c, personality=0x1f1ccb1d97a0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): calling personality function 0x1f1ccb1d97a0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): pc=0x1f1d108e77ef, start_ip=0x1f1d108e7330, func=_ZN12rustc_driver12run_compiler17h8fb3410f76685058E, lsda=0x1f1d10a7a9d4, personality=0x1f1ccb1d97a0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): calling personality function 0x1f1ccb1d97a0
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): _URC_CONTINUE_UNWIND
libuwind: unwind_phase1(ex_ojb=0x1f1d1d5a7000): unw_step() reached bottom => _URC_END_OF_STACK
fatal runtime error: failed to initiate panic, error 5
[New thread 259044]

Thread 2 received signal SIGABRT, Aborted.
[Switching to thread 259044]
thrkill () at -:3
3       -: No such file or directory.
(gdb) bt
#0  thrkill () at -:3
#1  0x00001f1d2ddb11ad in _libc_abort () at /usr/src/lib/libc/stdlib/abort.c:52
#2  0x00001f1ccb1bed31 in std::sys_common::util::abort::h076c0bb8451af0e6 () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/libstd-2ce1d6a76eafdd05.so
#3  0x00001f1ccb1cd57e in rust_panic () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/libstd-2ce1d6a76eafdd05.so
#4  0x00001f1ccb1cd4e6 in std::panicking::rust_panic_with_hook::hf2d11de5f905222b () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/libstd-2ce1d6a76eafdd05.so
#5  0x00001f1d7cc32d78 in std::panicking::begin_panic::h6afedfaffc3f9d2e () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/librustc-011439a32a903962.so
#6  0x00001f1d7ce2953d in rustc::session::early_error::h0187f9fdb6331d60 () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/librustc-011439a32a903962.so
#7  0x00001f1d108ebe6d in _$LT$rustc_driver..RustcDefaultCalls$u20$as$u20$rustc_driver..CompilerCalls$LT$$u27$a$GT$$GT$::no_input::hc57ca5476db31dac () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/librustc_driver-052b8dbae020bc96.so
#8  0x00001f1d108e77ef in rustc_driver::run_compiler::h8fb3410f76685058 () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/librustc_driver-052b8dbae020bc96.so
#9  0x00001f1d1080c26c in std::sys_common::backtrace::__rust_begin_short_backtrace::hd612f4587ecab5f6 () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/librustc_driver-052b8dbae020bc96.so
#10 0x00001f1ccb1d9c27 in __rust_maybe_catch_panic () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/libstd-2ce1d6a76eafdd05.so
#11 0x00001f1d10831c61 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hff49756066bea45b () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/librustc_driver-052b8dbae020bc96.so
#12 0x00001f1ccb1cc1a8 in std::sys::imp::thread::Thread::new::thread_start::h4f30b5b9f427f3ca () from /data/semarie/repos/openbsd/ports/pobj/rust-1.19.0/build-amd64/build/x86_64-unknown-openbsd/stage2/lib/libstd-2ce1d6a76eafdd05.so
#13 0x00001f1d8fca15ce in _rthread_start (v=0x0) at /usr/src/lib/librthread/rthread.c:115
#14 0x00001f1d2dd5dbdb in __tfork_thread () at /usr/src/lib/libc/arch/amd64/sys/tfork_thread.S:75
#15 0x0000000000000000 in ?? ()
(gdb) 
