
(lldb) bt
* thread #1, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x00000001000070b2 d`std::rt::lang_start_internal::h573771bdd5f2def7 [inlined] core::mem::zeroed::hc0abb49ba4fea050 at mod.rs:499:4 [opt]
    frame #1: 0x00000001000070af d`std::rt::lang_start_internal::h573771bdd5f2def7 [inlined] std::sys::unix::stack_overflow::imp::make_handler::hbf628ed912a2c133 at stack_overflow.rs:155 [opt]
    frame #2: 0x00000001000070af d`std::rt::lang_start_internal::h573771bdd5f2def7 [inlined] std::sys::unix::stack_overflow::imp::init::he5642e500424f708 at stack_overflow.rs:119 [opt]
    frame #3: 0x000000010000707b d`std::rt::lang_start_internal::h573771bdd5f2def7 at rt.rs:38 [opt]
    frame #4: 0x0000000100000d55 d`std::rt::lang_start::hba2a92a3b9483f06 + 53
    frame #5: 0x0000000100000dd2 d`main + 34
    frame #6: 0x0000000100001e10 d`std::thread::local::fast::destroy_value::h0b652cde96d37f2a at mod.rs:184
    frame #7: 0x0000000100000d7a d`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hda773a68d12ca370 + 10
    frame #8: 0x00000001000068f8 d`std::panicking::try::do_call::h92a7a402bb9bf6ba [inlined] std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::ha06528cd672f08b7 at rt.rs:52:12 [opt]
    frame #9: 0x00000001000068ec d`std::panicking::try::do_call::h92a7a402bb9bf6ba at panicking.rs:292 [opt]
    frame #10: 0x0000000100007eef d`__rust_maybe_catch_panic at lib.rs:78:7 [opt]
    frame #11: 0x00000001000071ae d`std::rt::lang_start_internal::h573771bdd5f2def7 [inlined] std::panicking::try::h1c0d44dceba304e7 at panicking.rs:270:12 [opt]
    frame #12: 0x000000010000717b d`std::rt::lang_start_internal::h573771bdd5f2def7 [inlined] std::panic::catch_unwind::h20a0db65f17b0ccd at panic.rs:394 [opt]
    frame #13: 0x000000010000717b d`std::rt::lang_start_internal::h573771bdd5f2def7 at rt.rs:51 [opt]
    frame #14: 0x0000000100000d55 d`std::rt::lang_start::hba2a92a3b9483f06 + 53
    frame #15: 0x0000000100000dd2 d`main + 34
    frame #16: 0x00007fff62aea3d5 libdyld.dylib`start + 1
