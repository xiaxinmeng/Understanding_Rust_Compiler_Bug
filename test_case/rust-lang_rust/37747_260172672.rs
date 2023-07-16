
(lldb) bt
* thread #1: tid = 0x5487c7, 0x000000010000700c lfail`std::sys::imp::stack_overflow::imp::make_handler [inlined] core::mem::zeroed<libc::unix::bsd::apple::stack_t> + 8 at mem.rs:334, queue = 'com.apple.main-thread', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x000000010000700c lfail`std::sys::imp::stack_overflow::imp::make_handler [inlined] core::mem::zeroed<libc::unix::bsd::apple::stack_t> + 8 at mem.rs:334 [opt]
    frame #1: 0x0000000100007004 lfail`std::sys::imp::stack_overflow::imp::make_handler + 4 at stack_overflow.rs:173 [opt]
    frame #2: 0x0000000100007bf3 lfail`std::rt::lang_start [inlined] std::sys::imp::stack_overflow::imp::init + 82 at stack_overflow.rs:134 [opt]
    frame #3: 0x0000000100007ba1 lfail`std::rt::lang_start + 177 at rt.rs:44 [opt]
