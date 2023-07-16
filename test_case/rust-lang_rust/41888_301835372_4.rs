
#0  std::sys::imp::init::oom_handler () at /checkout/src/libstd/sys/unix/mod.rs:89
#1  0x000055555556f4bd in alloc::oom::imp::oom () at /checkout/src/liballoc/oom.rs:41
#2  alloc::oom::oom () at /checkout/src/liballoc/oom.rs:27
#3  0x000055555555b973 in _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::allocate::h98ae98b690b5a3d6 ()
#4  0x000055555555b859 in _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::with_capacity::h646536492684a6cd
    ()
#5  0x000055555556f2eb in panic_unwind::__rust_maybe_catch_panic ()
    at /checkout/src/libpanic_unwind/lib.rs:98
#6  0x000055555555b815 in std::panicking::try::h9f2efeaaffe7c6c4 ()
#7  0x000055555555b469 in std::panic::catch_unwind::h3b627ac82596e62b ()
#8  0x000055555555b751 in std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::h99fd97c22f4dc633 ()
#9  0x000055555555c519 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h53150036c5a896a3 ()
#10 0x0000555555566626 in alloc::boxed::{{impl}}::call_once<(),()> ()
    at /checkout/src/liballoc/boxed.rs:658
#11 std::sys_common::thread::start_thread () at /checkout/src/libstd/sys_common/thread.rs:21
#12 std::sys::imp::thread::{{impl}}::new::thread_start ()
    at /checkout/src/libstd/sys/unix/thread.rs:84
#13 0x00007ffff77b56ba in start_thread () from /lib/x86_64-linux-gnu/libpthread.so.0
#14 0x00007ffff72d582d in clone () from /lib/x86_64-linux-gnu/libc.so.6
