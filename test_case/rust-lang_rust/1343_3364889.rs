
#0  0xf7ef976e in check_stack_alignment () from /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/test/run-fail/../../stage1/lib/rustc/i686-unknown-linux
-gnu/lib/librustrt.so
#1  0xf7d92936 in memmove () at ../sysdeps/i386/i686/multiarch/memmove.S:42
#2  0xf7fdd900 in ?? ()
#3  0xf7ed9eb5 in upcall_fail (expr=0x8049090 "explicit failure", file=0x80490b0 "../src/test/run-fail/alt-bot-fail.rs", line=7) at ../src/rt/rust_upcall.cpp:9
1
#4  0x08048bd0 in main ()
#5  0x08048c21 in _rust_main ()
#6  0xf7ed49ce in task_start_wrapper (a=0x8052d3c) at ../src/rt/rust_task.cpp:354
#7  0x00000000 in ?? ()
