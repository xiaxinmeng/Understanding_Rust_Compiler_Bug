
# lldb /Users/servo/test/text_util
(lldb) target create "/Users/servo/test/text_util"
2018-05-18 12:50:51.381 lldb[48794:3611387704] Metadata.framework [Error]: couldn't get the client port
Current executable set to '/Users/servo/test/text_util' (x86_64).
(lldb) r
Process 49139 launched: '/Users/servo/test/text_util' (x86_64)

running 1 test
Process 49139 stopped
* thread #2: tid = 0xd741685b, 0x000000010005d312 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] core::ptr::swap_nonoverlapping_bytes::h5ceec518bedbc8b5 + 28 at ptr.rs:237, name = 'test_transform_compress_none', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x000000010005d312 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] core::ptr::swap_nonoverlapping_bytes::h5ceec518bedbc8b5 + 28 at ptr.rs:237
(lldb) bt
* thread #2: tid = 0xd741685b, 0x000000010005d312 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] core::ptr::swap_nonoverlapping_bytes::h5ceec518bedbc8b5 + 28 at ptr.rs:237, name = 'test_transform_compress_none', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x000000010005d312 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] core::ptr::swap_nonoverlapping_bytes::h5ceec518bedbc8b5 + 28 at ptr.rs:237
    frame #1: 0x000000010005d2f6 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] core::ptr::swap_nonoverlapping::h135f714f781a3d4a at ptr.rs:187
    frame #2: 0x000000010005d2f6 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] core::mem::swap::ha8a5fb0906ce1619 at mem.rs:634
    frame #3: 0x000000010005d2f6 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] core::mem::replace::hc5999657bde25f00 at mem.rs:691
    frame #4: 0x000000010005d2f6 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::init::h03c530c70809d764 at local.rs:270
    frame #5: 0x000000010005d2f6 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::try_with::h1ddab9e8e4e02c71 + 34 at local.rs:296
    frame #6: 0x000000010005d2d4 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::h7ad9abd4ee126f73 at local.rs:248
    frame #7: 0x000000010005d2d4 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 [inlined] std::panicking::update_panic_count::h8ff9c302601a529d at panicking.rs:240
    frame #8: 0x000000010005d2d4 text_util`std::panicking::panicking::hbf60d20c3ea01bc0 + 4 at panicking.rs:317
    frame #9: 0x00000001000176b3 text_util`std::sys_common::backtrace::__rust_begin_short_backtrace::h5ce7af51dff78437 [inlined] std::thread::panicking::ha8ec11437d5dda69 + 563 at mod.rs:650
    frame #10: 0x00000001000176ae text_util`std::sys_common::backtrace::__rust_begin_short_backtrace::h5ce7af51dff78437 [inlined] std::sys_common::poison::Flag::borrow::h4e2e930fbcd29fe9 at poison.rs:36
    frame #11: 0x00000001000176ae text_util`std::sys_common::backtrace::__rust_begin_short_backtrace::h5ce7af51dff78437 [inlined] _$LT$std..sync..mutex..MutexGuard$LT$$u27$mutex$C$$u20$T$GT$$GT$::new::hf958532833c3eeee at mutex.rs:426
    frame #12: 0x00000001000176ae text_util`std::sys_common::backtrace::__rust_begin_short_backtrace::h5ce7af51dff78437 [inlined] _$LT$std..sync..mutex..Mutex$LT$T$GT$$GT$::lock::h08aa85ea884e0a79 + 9 at mutex.rs:231
    frame #13: 0x00000001000176a5 text_util`std::sys_common::backtrace::__rust_begin_short_backtrace::h5ce7af51dff78437 [inlined] test::run_test::run_test_inner::_$u7b$$u7b$closure$u7d$$u7d$::h293891f148241b3f + 508 at lib.rs:1414
    frame #14: 0x00000001000174a9 text_util`std::sys_common::backtrace::__rust_begin_short_backtrace::h5ce7af51dff78437 + 41 at backtrace.rs:136
    frame #15: 0x000000010002e6d8 text_util`std::panicking::try::do_call::h317793a359ef8eae [inlined] std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hce2a129863ba037b + 40 at mod.rs:409
    frame #16: 0x000000010002e6c2 text_util`std::panicking::try::do_call::h317793a359ef8eae [inlined] _$LT$std..panic..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::ha38aec20aaf8023b at panic.rs:305
    frame #17: 0x000000010002e6c2 text_util`std::panicking::try::do_call::h317793a359ef8eae + 18 at panicking.rs:310
    frame #18: 0x0000000100068f9f text_util`__rust_maybe_catch_panic + 31 at lib.rs:105
    frame #19: 0x0000000100018fa5 text_util`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hbda32c8bc14400fa [inlined] std::panicking::try::hd50d66bdaa9b80f6 + 57 at panicking.rs:289
    frame #20: 0x0000000100018f6c text_util`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hbda32c8bc14400fa [inlined] std::panic::catch_unwind::h23f85cf47b3beb69 + 7 at panic.rs:374
    frame #21: 0x0000000100018f65 text_util`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hbda32c8bc14400fa [inlined] std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::hb9992794009509d0 + 55 at mod.rs:408
    frame #22: 0x0000000100018f2e text_util`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hbda32c8bc14400fa + 46 at boxed.rs:638
    frame #23: 0x000000010005a178 text_util`std::sys_common::thread::start_thread::h3a2a1586806f3926 [inlined] _$LT$alloc..boxed..Box$LT$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::he272aed47e96b55c + 136 at boxed.rs:648
    frame #24: 0x000000010005a175 text_util`std::sys_common::thread::start_thread::h3a2a1586806f3926 + 133 at thread.rs:24
    frame #25: 0x000000010004c0a9 text_util`std::sys::unix::thread::Thread::new::thread_start::h657bd8405a5d884c + 9 at thread.rs:90
    frame #26: 0x00007fff95aed05a libsystem_pthread.dylib`_pthread_body + 131
    frame #27: 0x00007fff95aecfd7 libsystem_pthread.dylib`_pthread_start + 176
    frame #28: 0x00007fff95aea3ed libsystem_pthread.dylib`thread_start + 13
