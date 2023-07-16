
(lldb) bt
* thread #2, name = 'bench_httparse_short', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x0000000100050217 parse-45f0c687ce1cb6ef`std::sys_common::thread_info::set::h653837fe234bdc81 [inlined] core::ptr::swap_nonoverlapping_bytes::h0b169aa8ef3bf4f2 at ptr.rs:221 [opt]
    frame #1: 0x000000010005020e parse-45f0c687ce1cb6ef`std::sys_common::thread_info::set::h653837fe234bdc81 [inlined] core::ptr::swap_nonoverlapping::hca32daf491eb7d79 at ptr.rs:187 [opt]
    frame #2: 0x000000010005020e parse-45f0c687ce1cb6ef`std::sys_common::thread_info::set::h653837fe234bdc81 [inlined] core::mem::swap::he62a32c44c5d3a12 at mem.rs:634 [opt]
    frame #3: 0x000000010005020e parse-45f0c687ce1cb6ef`std::sys_common::thread_info::set::h653837fe234bdc81 [inlined] core::mem::replace::hee354da570f8b6a1 at mem.rs:691 [opt]
    frame #4: 0x000000010005020e parse-45f0c687ce1cb6ef`std::sys_common::thread_info::set::h653837fe234bdc81 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::init::h8241a6fabfe63004 at local.rs:270 [opt]
    frame #5: 0x000000010005020e parse-45f0c687ce1cb6ef`std::sys_common::thread_info::set::h653837fe234bdc81 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::try_with::h019266c217631df0 at local.rs:296 [opt]
    frame #6: 0x00000001000501ba parse-45f0c687ce1cb6ef`std::sys_common::thread_info::set::h653837fe234bdc81 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::h977b626335ede091 at local.rs:248 [opt]
    frame #7: 0x00000001000501ba parse-45f0c687ce1cb6ef`std::sys_common::thread_info::set::h653837fe234bdc81 at thread_info.rs:47 [opt]
    frame #8: 0x000000010002cb88 parse-45f0c687ce1cb6ef`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h6f23826ab9e7d513 [inlined] std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::h0bb9a60731c9f04c at mod.rs:406 [opt]
    frame #9: 0x000000010002cb1b parse-45f0c687ce1cb6ef`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h6f23826ab9e7d513 at boxed.rs:638 [opt]
    frame #10: 0x00000001000422b8 parse-45f0c687ce1cb6ef`std::sys::unix::thread::Thread::new::thread_start::h0c1394726addc699 [inlined] _$LT$alloc..boxed..Box$LT$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hbdc518a2bdf15123 at boxed.rs:648 [opt]
    frame #11: 0x00000001000422b5 parse-45f0c687ce1cb6ef`std::sys::unix::thread::Thread::new::thread_start::h0c1394726addc699 [inlined] std::sys_common::thread::start_thread::h8d1134075e93d8df at thread.rs:24 [opt]
    frame #12: 0x0000000100042240 parse-45f0c687ce1cb6ef`std::sys::unix::thread::Thread::new::thread_start::h0c1394726addc699 at thread.rs:90 [opt]
    frame #13: 0x00007fff5ef8d661 libsystem_pthread.dylib`_pthread_body + 340
    frame #14: 0x00007fff5ef8d50d libsystem_pthread.dylib`_pthread_start + 377
    frame #15: 0x00007fff5ef8cbf9 libsystem_pthread.dylib`thread_start + 13
