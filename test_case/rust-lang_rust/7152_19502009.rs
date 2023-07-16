
Breakpoint 1, upcall_fail (expr=0x100095eb0 "option::unwrap none", file=0x100095ed0 "/Users/jason/src/rust/src/libstd/option.rs", line=268) at rust_upcall.cpp:126
126             size_t line) {
(gdb) bt
#0  upcall_fail (expr=0x100095eb0 "option::unwrap none", file=0x100095ed0 "/Users/jason/src/rust/src/libstd/option.rs", line=268) at rust_upcall.cpp:126
#1  0x00000001001b7a83 in sys::begin_unwind_::_89e154cd0915671::_07pre ()
#2  0x00000001001b7b52 in sys::__extensions__::fail_with::anon::anon::expr_fn_22379 ()
#3  0x00000001001062a4 in sys::__extensions__::meth_10847::fail_with::_db4c44d01ce4116::_07pre ()
#4  0x0000000100079ca3 in comm::__extensions__::send_9140::_6efa6b5d4827a71::_00 ()
#5  0x000000010006d155 in server::anon::anon::expr_fn_8556 ()
#6  0x000000010009015d in __morestack ()
Previous frame inner to this frame (gdb could not unwind past this frame)
Current language:  auto; currently c++
