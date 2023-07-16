
[Switching to process 15175 thread 0x2207]

Breakpoint 1, rust_task_fail (task=0x108000420, expr=0x100095eb0 "option::unwrap none", file=0x100095ed0 "/Users/jason/src/rust/src/libstd/option.rs", line=268) at rust_task.cpp:90
90                 size_t line) {
(gdb) bt
#0  rust_task_fail (task=0x108000420, expr=0x100095eb0 "option::unwrap none", file=0x100095ed0 "/Users/jason/src/rust/src/libstd/option.rs", line=268) at rust_task.cpp:90
#1  0x000000010096ce67 in rust_task::fail (this=<value temporarily unavailable, due to optimizations>, expr=<value temporarily unavailable, due to optimizations>, file=<value temporarily unavailable, due to optimizations>, line=<value temporarily unavailable, due to optimizations>) at rust_task.cpp:328
#2  0x000000010097cd51 in __morestack () at rust_task.cpp:1327
#3  0x000000010096d5de in rust_task::call_on_c_stack (this=0x108000420, args=0x1009a2588,     fn_ptr=0x100095ed0) at rust_task.h:484
#4  0x000000010096e7d6 in upcall_fail (expr=0x100095eb0 "option::unwrap none", file=0x100095ed0 "/Users/jason/src/rust/src/libstd/option.rs", line=268) at rust_upcall.cpp:51
#5  0x00000001001b7a83 in sys::begin_unwind_::_89e154cd0915671::_07pre ()
#6  0x00000001001b7b52 in sys::__extensions__::fail_with::anon::anon::expr_fn_22379 ()
#7  0x00000001001062a4 in sys::__extensions__::meth_10847::fail_with::_db4c44d01ce4116::_07pre ()
#8  0x0000000100079ca3 in comm::__extensions__::send_9140::_6efa6b5d4827a71::_00 ()
#9  0x000000010006d155 in server::anon::anon::expr_fn_8556 ()
#10 0x000000010009015d in __morestack ()
Previous frame inner to this frame (gdb could not unwind past this frame)
Current language:  auto; currently c++
