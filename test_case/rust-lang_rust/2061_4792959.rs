
rust: task 6598840 ran out of stack
terminate called after throwing an instance of 'rust_task*'
==14512== 136 bytes in 1 blocks are possibly lost in loss record 10 of 23
==14512==    at 0x4C28FAC: malloc (vg_replace_malloc.c:236)
==14512==    by 0x633E232: __cxa_allocate_exception (in /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.14)
==14512==    by 0x5472688: rust_task::fail() (rust_task.cpp:256)
==14512==    by 0x5472CAC: rust_task::new_stack(unsigned long) (rust_task.cpp:504)
==14512==    by 0x5485CB8: ??? (in /home/banderson/Dev/rust2/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so)
==14512== 
