
#0  rust_log::trace_ln () at /Users/linuxfood/Projects/rust/src/rt/rust_log.cpp:70
#1  0x000000010007f0d3 in rust_log::trace_ln (this=0x100815c28, task=0x10009f8d0, level=0, message=0x100400340 "\"this is a warning statement.\"") at rust_log.cpp:111
#2  0x00000001000786a9 in rust_scheduler::log (this=0x100815c00, task=0x10009f8d0, level=0, fmt=0x0) at rust_scheduler.cpp:76
#3  0x00000001000829cb in shape_log_type (tydesc=0x1000020f0, data=0x1010055c8 "P", level=1) at rust_shape.cpp:571
#4  0x000000010008bf59 in __morestack () at new_allocator.h:107
#5  0x000000010007d5bb in upcall_log_type (tydesc=0x1003ffa30, data=0x10009f8d0 "", level=0) at context.h:58
#6  0x00000001000015e1 in main::_7c7a42e36d1ce49f ()
#7  0x0000000100001c85 in __morestack ()
Previous frame inner to this frame (gdb could not unwind past this frame)
