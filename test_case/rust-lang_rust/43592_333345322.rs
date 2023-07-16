
ct-tools(62565,0x70000a072000) malloc: *** error for object 0x100c021e8: incorrect checksum for freed object - object was probably modified after being freed.
*** set a breakpoint in malloc_error_break to debug
Process 62565 stopped
* thread #2, stop reason = signal SIGABRT
    frame #0: 0x00007fff6eadffce libsystem_kernel.dylib`__pthread_kill + 10
libsystem_kernel.dylib`__pthread_kill:
->  0x7fff6eadffce <+10>: jae    0x7fff6eadffd8            ; <+20>
    0x7fff6eadffd0 <+12>: movq   %rax, %rdi
    0x7fff6eadffd3 <+15>: jmp    0x7fff6ead776c            ; cerror_nocancel
    0x7fff6eadffd8 <+20>: retq
Target 0: (ct-tools) stopped.
(lldb) bt
* thread #2, stop reason = signal SIGABRT
  * frame #0: 0x00007fff6eadffce libsystem_kernel.dylib`__pthread_kill + 10
    frame #1: 0x00007fff6ec1d150 libsystem_pthread.dylib`pthread_kill + 333
    frame #2: 0x00007fff6ea3c32a libsystem_c.dylib`abort + 127
    frame #3: 0x00007fff6eb44b28 libsystem_malloc.dylib`szone_error + 596
    frame #4: 0x00007fff6eb3a76b libsystem_malloc.dylib`tiny_free_list_remove_ptr + 298
    frame #5: 0x00007fff6eb4fac8 libsystem_malloc.dylib`tiny_free_no_lock + 1450
    frame #6: 0x00007fff6eb50254 libsystem_malloc.dylib`free_tiny + 628
    frame #7: 0x00007fff6ea8a6c2 libsystem_info.dylib`freeaddrinfo + 66
    frame #8: 0x000000010038181d ct-tools`std::net::addr::resolve_socket_addr [inlined] std::sys_common::net::{{impl}}::drop at net.rs:160 [opt]
    frame #9: 0x0000000100381811 ct-tools`std::net::addr::resolve_socket_addr [inlined] core::ptr::drop_in_place<std::sys_common::net::LookupHost> at ptr.rs:61 [opt]
    frame #10: 0x0000000100381811 ct-tools`std::net::addr::resolve_socket_addr [inlined] core::ptr::drop_in_place<std::net::LookupHost> at ptr.rs:61 [opt]
    frame #11: 0x0000000100381811 ct-tools`std::net::addr::resolve_socket_addr [inlined] core::ptr::drop_in_place<core::iter::Map<std::net::LookupHost, closure>> at ptr.rs:61 [opt]
    frame #12: 0x0000000100381811 ct-tools`std::net::addr::resolve_socket_addr [inlined] alloc::vec::{{impl}}::extend_desugared<std::net::addr::SocketAddr,core::iter::Map<std::net::LookupHost, closure>> at vec.rs:1915 [opt]
    frame #13: 0x00000001003815d0 ct-tools`std::net::addr::resolve_socket_addr [inlined] alloc::vec::{{impl}}::spec_extend<std::net::addr::SocketAddr,core::iter::Map<std::net::LookupHost, closure>> at vec.rs:1800 [opt]
    frame #14: 0x00000001003815d0 ct-tools`std::net::addr::resolve_socket_addr [inlined] alloc::vec::{{impl}}::from_iter<std::net::addr::SocketAddr,core::iter::Map<std::net::LookupHost, closure>> at vec.rs:1795 [opt]
    frame #15: 0x0000000100381400 ct-tools`std::net::addr::resolve_socket_addr [inlined] alloc::vec::{{impl}}::from_iter<std::net::addr::SocketAddr,core::iter::Map<std::net::LookupHost, closure>> at vec.rs:1696 [opt]
    frame #16: 0x0000000100381400 ct-tools`std::net::addr::resolve_socket_addr [inlined] core::iter::iterator::Iterator::collect<core::iter::Map<std::net::LookupHost, closure>,alloc::vec::Vec<std::net::addr::SocketAddr>> at iterator.rs:1298 [opt]
    frame #17: 0x0000000100381400 ct-tools`std::net::addr::resolve_socket_addr at addr.rs:850 [opt]
    frame #18: 0x0000000100381a9c ct-tools`std::net::addr::{{impl}}::to_socket_addrs at addr.rs:870 [opt]
    frame #19: 0x00000001002638ef ct-tools`std::panicking::try::do_call::h98e985df9219ceeb + 479
    frame #20: 0x000000010039c44d ct-tools`panic_unwind::__rust_maybe_catch_panic at lib.rs:99 [opt]
    frame #21: 0x0000000100261b32 ct-tools`_$LT$futures_cpupool..MySender$LT$F$C$$u20$core..result..Result$LT$$LT$F$u20$as$u20$futures..future..Future$GT$..Item$C$$u20$$LT$F$u20$as$u20$futures..future..Future$GT$..Error$GT$$GT$$u20$as$u20$futures..future..Future$GT$::poll::h5fdc3798a778b180 + 626
    frame #22: 0x0000000100298a90 ct-tools`futures::task_impl::std::Run::run::h94868cd32b108ec9 + 448
    frame #23: 0x000000010028f553 ct-tools`std::sys_common::backtrace::__rust_begin_short_backtrace::hf0ef3b99d4e69109 + 259
    frame #24: 0x000000010039c44d ct-tools`panic_unwind::__rust_maybe_catch_panic at lib.rs:99 [opt]
    frame #25: 0x0000000100293650 ct-tools`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::ha45b02055eb8de54 + 144
    frame #26: 0x000000010038e55c ct-tools`std::sys::imp::thread::{{impl}}::new::thread_start [inlined] alloc::boxed::{{impl}}::call_once<(),()> at boxed.rs:738 [opt]
    frame #27: 0x000000010038e559 ct-tools`std::sys::imp::thread::{{impl}}::new::thread_start [inlined] std::sys_common::thread::start_thread at thread.rs:24 [opt]
    frame #28: 0x000000010038e4de ct-tools`std::sys::imp::thread::{{impl}}::new::thread_start at thread.rs:90 [opt]
    frame #29: 0x00007fff6ec1a6c1 libsystem_pthread.dylib`_pthread_body + 340
    frame #30: 0x00007fff6ec1a56d libsystem_pthread.dylib`_pthread_start + 377
    frame #31: 0x00007fff6ec19c5d libsystem_pthread.dylib`thread_start + 13
