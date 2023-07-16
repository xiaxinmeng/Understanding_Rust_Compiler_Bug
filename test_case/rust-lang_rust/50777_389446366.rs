
#0  0x00007fffe6a0d060 in ?? ()
#1  0x00005555557a3d8e in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hb9eec09583bb707c ()
#2  0x00007ffff3a94e03 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h53f037bb127012d1 ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libtest-c79d24ea4c4766b2.so
#3  0x00007ffff23b091a in __rust_maybe_catch_panic ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libstd-72fc73250c737cd2.so
#4  0x00007ffff3aab8ee in std::sys_common::backtrace::__rust_begin_short_backtrace::h25f25e1e96dde5fb ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libtest-c79d24ea4c4766b2.so
#5  0x00007ffff3ac8185 in std::panicking::try::do_call::h1e947f9784f95f13 ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libtest-c79d24ea4c4766b2.so
#6  0x00007ffff23b091a in __rust_maybe_catch_panic ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libstd-72fc73250c737cd2.so
#7  0x00007ffff3aad5c7 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h6d53f833c8f8ba98 ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libtest-c79d24ea4c4766b2.so
#8  0x00007ffff23a332b in std::sys_common::thread::start_thread::h12aa3fa026a33681 ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libstd-72fc73250c737cd2.so
#9  0x00007ffff236e676 in std::sys::unix::thread::Thread::new::thread_start::hbcfc644f83802907 ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libstd-72fc73250c737cd2.so
#10 0x00007ffff20ff6ba in start_thread (arg=0x7fffe6fff700) at pthread_create.c:333
#11 0x00007ffff1c1f41d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
