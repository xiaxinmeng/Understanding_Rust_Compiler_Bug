
Program received signal SIGABRT, Aborted.
0x00007fff90707d46 in __kill ()
(gdb) bt
#0  0x00007fff90707d46 in __kill ()
#1  0x00007fff89d96df0 in abort ()
#2  0x00000001001495a6 in rt::util::abort::abort::_7c12263797ed078::_0$x2e8$x2dpre ()
#3  0x0000000100076af6 in rt::util::abort::_f2c3fded91cbadbb::_0$x2e8$x2dpre ()
#4  0x00000001000d871c in sys::begin_unwind_::_89e154cd0915671::_0$x2e8$x2dpre ()
#5  0x00000001000d8d22 in sys::__extensions__::fail_with::anon::anon::expr_fn_26302 ()
#6  0x00000001000d8558 in c_str::ToCStr::with_c_str_26291::_c6798931b183a7::_0$x2e8$x2dpre ()
#7  0x00000001000d8cd1 in sys::__extensions__::fail_with::anon::expr_fn_26300 ()
#8  0x00000001000d8558 in c_str::ToCStr::with_c_str_26291::_c6798931b183a7::_0$x2e8$x2dpre ()
#9  0x000000010005ab4c in sys::__extensions__::meth_14197::fail_with::_db4c44d01ce4116::_0$x2e8$x2dpre ()
#10 0x0000000100149541 in rt::util::default_sched_threads::_806083deb827ec::_0$x2e8$x2dpre ()
#11 0x000000010014b16f in rt::run_::_82e8c355ab8d949f::_0$x2e8$x2dpre ()
#12 0x00000001000eef7b in unstable::lang::start::_76d6c774aa357c7a::_0$x2e8$x2dpre ()
#13 0x000000010000147b in main ()
