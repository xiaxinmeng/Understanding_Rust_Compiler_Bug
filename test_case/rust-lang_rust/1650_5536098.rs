
Catchpoint 1 (exception thrown), 0x00007ffff66a0ff0 in __cxa_throw ()
   from /usr/lib/x86_64-linux-gnu/libstdc++.so.6
(gdb) bt
#0  0x00007ffff66a0ff0 in __cxa_throw () from /usr/lib/x86_64-linux-gnu/libstdc++.so.6
#1  0x00007ffff754e35d in rust_task::fail (this=0x60a120)
    at /home/gareth/projects/rust/src/rt/rust_task.cpp:269
#2  0x00007ffff7563d79 in __morestack ()
   from /home/gareth/projects/tests/../../../../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so
#3  0x00007ffff7551321 in call_on_c_stack (fn_ptr=0x7ffff75506a0, args=0x60af60, this=0x60a120)
    at /home/gareth/projects/rust/src/rt/rust_task.h:352
#4  call_upcall_on_c_stack (fn_ptr=0x7ffff75506a0, args=0x60af60)
    at /home/gareth/projects/rust/src/rt/rust_upcall.cpp:50
#5  upcall_fail (expr=<optimized out>, file=<optimized out>, line=<optimized out>)
    at /home/gareth/projects/rust/src/rt/rust_upcall.cpp:129
#6  0x0000000000400af7 in z () at fleh.rs:1
#7  0x0000000000400bd6 in y () at fleh.rs:3
#8  0x0000000000400c16 in x () at fleh.rs:5
#9  0x0000000000400c56 in main () at fleh.rs:8
#10 0x0000000000400cac in _rust_main ()
#11 0x00007ffff754ec4b in task_start_wrapper (a=<optimized out>)
    at /home/gareth/projects/rust/src/rt/rust_task.cpp:141
#12 0x0000000000000000 in ?? ()
