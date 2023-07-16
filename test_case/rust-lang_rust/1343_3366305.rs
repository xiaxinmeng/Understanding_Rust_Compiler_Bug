
command: /usr/bin/valgrind --leak-check=full --error-exitcode=100 --quiet --suppressions=../src/etc/x86.supp x86_64-unknown-linux-gnu/test/run-fail/morestack4.stage1-i686-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
vex x86->IR: unhandled instruction bytes: 0x6F 0x29 0xAC 0x4
==15754== Thread 4:
==15754== Invalid read of size 1
==15754==    at 0x804AFF4: ??? (in /home/brian/Dev/rust/build/x86_64-unknown-linux-gnu/test/run-fail/morestack4.stage1-i686-unknown-linux-gnu)
==15754==    by 0x728DAD7: ???
==15754==  Address 0x728e75c is 2,644 bytes inside a block of size 2,836 alloc'd
==15754==    at 0x48DDBD3: malloc (vg_replace_malloc.c:236)
==15754==    by 0x4AC6DD0: rust_srv::malloc(unsigned int) (rust_srv.cpp:18)
==15754==    by 0x4AE1208: memory_region::malloc(unsigned int, char const*, bool) (memory_region.cpp:104)
==15754==    by 0x4ABDACC: rust_task::malloc(unsigned int, char const*, type_desc*) (rust_task.cpp:530)
==15754==    by 0x4ABC349: new_stk(rust_scheduler*, rust_task*, unsigned int) (rust_task.cpp:183)
==15754==    by 0x4ABCAEE: rust_task::rust_task(rust_scheduler*, rust_task_list*, rust_task*, char const*) (rust_task.cpp:261)
==15754==    by 0x4ABB2FB: rust_scheduler::create_task(rust_task*, char const*) (rust_scheduler.cpp:340)
