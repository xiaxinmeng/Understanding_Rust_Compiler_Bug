
(gdb) bt
#0  __syscall () at src/internal/x86_64/syscall.s:13
#1  0x000000000045e3f8 in nanosleep (req=<optimized out>, rem=<optimized out>) at src/time/nanosleep.c:7
#2  0x0000000000445833 in std::sys::unix::thread::Thread::sleep::h5a6a14352632043c ()
#3  0x000000000040032b in x::main::h33e0397ddc129856 ()
#4  0x0000000000400403 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h7e9e7f9d5763a160 ()
#5  0x000000000043f6f3 in std::panicking::try::do_call::h652c91b274df4c69 ()
#6  0x000000000045017a in __rust_maybe_catch_panic ()
#7  0x000000000043e5d7 in std::rt::lang_start_internal::h2e87c2c21b125ac0 ()
#8  0x00000000004003e7 in std::rt::lang_start::h57093f2887229336 ()
#9  0x000000000040034e in main ()
