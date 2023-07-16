
#0  rust_task::new_stack (this=0x103500000, requested_sz=24) at /Users/pnkfelix/Dev/Mozilla/rust.git/src/rt/rust_task.cpp:532
#1  0x000000010332643f in __morestack ()
#2  0x0000000103317771 in rust_task::return_c_stack () at /Users/pnkfelix/Dev/Mozilla/rust.git/src/rt/rust_task.h:472
#3  0x0000000103317771 in rust_task::call_on_c_stack (this=0x103500000, args=0x81fb80, fn_ptr=0x100000) at rust_task.h:477
#4  0x0000000103318a50 in rust_task::next_stack (this=<value temporarily unavailable, due to optimizations>, stk_sz=<value temporarily unavailable, due to optimizations>, args_addr=0x105525130, args_sz=<value temporarily unavailable, due to optimizations>) at rust_task.h:588
#5  0x000000010017afb9 in __morestack ()
#6  0x00000001000a2394 in rand::__extensions__::next_12073::_4b24ebc0314a72c::_07pre ()
#7  0x00000001000a23b0 in rand::__extensions__::next_12073::_4b24ebc0314a72c::_07pre ()
#8  0x00000001000a23b0 in rand::__extensions__::next_12073::_4b24ebc0314a72c::_07pre ()
...
#32764 0x00000001000a23b0 in rand::__extensions__::next_12073::_4b24ebc0314a72c::_07pre ()
#32765 0x00000001000a23b0 in rand::__extensions__::next_12073::_4b24ebc0314a72c::_07pre ()
#32766 0x000000010017afe7 in __morestack ()
