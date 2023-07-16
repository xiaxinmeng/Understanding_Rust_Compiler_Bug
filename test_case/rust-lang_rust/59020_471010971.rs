
* thread #2
  * frame #0: 0x00007fff5d073a46 libsystem_kernel.dylib`__psynch_mutexwait + 10
    frame #1: 0x00007fff5d23bb9d libsystem_pthread.dylib`_pthread_mutex_lock_wait + 83
    frame #2: 0x00007fff5d2394c8 libsystem_pthread.dylib`_pthread_mutex_lock_slow + 253
    frame #3: 0x0000000100008c05 foo`std::thread::park::h40cbaa717417ef95 [inlined] std::sys::unix::mutex::Mutex::lock::h1c54f0b7e37dd0a3 at mutex.rs:56 [opt]
    frame #4: 0x0000000100008c00 foo`std::thread::park::h40cbaa717417ef95 [inlined] std::sys_common::mutex::Mutex::raw_lock::hecb01a8a1467a3f9 at mutex.rs:36 [opt]
    frame #5: 0x0000000100008c00 foo`std::thread::park::h40cbaa717417ef95 [inlined] _$LT$std..sync..mutex..Mutex$LT$T$GT$$GT$::lock::h012f35fcc06840a7 at mutex.rs:220 [opt]
    frame #6: 0x0000000100008bfc foo`std::thread::park::h40cbaa717417ef95 at mod.rs:891 [opt]
    frame #7: 0x000000010000d4ed foo`std::sync::once::Once::call_inner::h0b29d64214dba756 at once.rs:421 [opt]
    frame #8: 0x000000010000d7ed foo`std::time::Instant::duration_since::h6b1dd60fca9d26e8 [inlined] std::sync::once::Once::call_once::h15f56e63f2b5f3bd at once.rs:220 [opt]
    frame #9: 0x000000010000d7c8 foo`std::time::Instant::duration_since::h6b1dd60fca9d26e8 [inlined] std::sys::unix::time::inner::info::he0ee845c3bb0a3ac at time.rs:242 [opt]
    frame #10: 0x000000010000d7c8 foo`std::time::Instant::duration_since::h6b1dd60fca9d26e8 [inlined] std::sys::unix::time::inner::Instant::sub_instant::h2f90455c1f5f50e3 at time.rs:154 [opt]
    frame #11: 0x000000010000d7c8 foo`std::time::Instant::duration_since::h6b1dd60fca9d26e8 at time.rs:218 [opt]
    frame #12: 0x0000000100012255 foo`std::sys::unix::condvar::Condvar::wait_timeout::hdd4a51d3f762f929 [inlined] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Sub$GT$::sub::h0703e52941cc9b4d at time.rs:307 [opt]
    frame #13: 0x000000010001224d foo`std::sys::unix::condvar::Condvar::wait_timeout::hdd4a51d3f762f929 [inlined] std::time::Instant::elapsed::h544a171a1c00e632 at time.rs:242 [opt]
    frame #14: 0x0000000100012240 foo`std::sys::unix::condvar::Condvar::wait_timeout::hdd4a51d3f762f929 at condvar.rs:162 [opt]
    frame #15: 0x0000000100008fe8 foo`std::thread::park_timeout::h730ea3969ae2f6b8 [inlined] std::sys_common::condvar::Condvar::wait_timeout::h8f2d65bc554e47d5 at condvar.rs:51 [opt]
    frame #16: 0x0000000100008fdd foo`std::thread::park_timeout::h730ea3969ae2f6b8 [inlined] std::sync::condvar::Condvar::wait_timeout::h552dfa042e4361fb at condvar.rs:405 [opt]
    frame #17: 0x0000000100008fc3 foo`std::thread::park_timeout::h730ea3969ae2f6b8 at mod.rs:1003 [opt]
    frame #18: 0x0000000100003203 foo`foo::main::_$u7b$$u7b$closure$u7d$$u7d$::h014edfb0ea6d8161((null)=closure {

}) at main.rs:9
    frame #19: 0x0000000100003bed foo`std::sys_common::backtrace::__rust_begin_short_backtrace::hfdb85b8c915ae023(f=closure {

}) at backtrace.rs:135
    frame #20: 0x00000001000068fd foo`std::thread::Builder::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hcb677bec99ca4490 at mod.rs:469
    frame #21: 0x00000001000057ed foo`_$LT$std..panic..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hc80622d5e14b1c63(self=AssertUnwindSafe<closure>(closure {

}), _args=<unavailable>) at panic.rs:309
    frame #22: 0x00000001000050ca foo`std::panicking::try::do_call::hcae109a146a05cf6(data=&0x70000ef8bd38) at panicking.rs:297
    frame #23: 0x000000010001384f foo`__rust_maybe_catch_panic at lib.rs:92 [opt]
    frame #24: 0x0000000100004f33 foo`std::panicking::try::h4483321badd4a86e(f=AssertUnwindSafe<closure>(closure {

})) at panicking.rs:276
    frame #25: 0x000000010000583d foo`std::panic::catch_unwind::h9d60a569083fc382(f=AssertUnwindSafe<closure>(closure {

})) at panic.rs:388
    frame #26: 0x0000000100006766 foo`std::thread::Builder::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::hed8c8ea5ca4cd51c at mod.rs:468
    frame #27: 0x0000000100006abf foo`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h781bdd77c151bf87(self=&0x100300380, args=<unavailable>) at boxed.rs:734
    frame #28: 0x000000010001324c foo`std::sys::unix::thread::Thread::new::thread_start::h2ccd51efe09a6d88 [inlined] _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf680f30793070da1 at boxed.rs:744 [opt]
    frame #29: 0x0000000100013249 foo`std::sys::unix::thread::Thread::new::thread_start::h2ccd51efe09a6d88 [inlined] std::sys_common::thread::start_thread::h30434b5681199bc5 at thread.rs:14 [opt]
    frame #30: 0x00000001000131ce foo`std::sys::unix::thread::Thread::new::thread_start::h2ccd51efe09a6d88 at thread.rs:81 [opt]
    frame #31: 0x00007fff5d23b661 libsystem_pthread.dylib`_pthread_body + 340
    frame #32: 0x00007fff5d23b50d libsystem_pthread.dylib`_pthread_start + 377
    frame #33: 0x00007fff5d23abf9 libsystem_pthread.dylib`thread_start + 13
