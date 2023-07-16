
==8818== Thread 9:
==8818== Invalid read of size 8
==8818==    at 0x4E7AC3C: str::hash::_31827ae260e7bf7b (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-14bd852465126fe7-0.1.so)
==8818==    by 0x51F68C4: map::chained::insert::_6cad5a231a3253ac (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-79ca5fac56b63fde-0.1.so)
==8818==    by 0x519FCC5: _ZN3map7chained7hashmap5168726insert17_76e8354b7f8b17bdE (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-79ca5fac56b63fde-0.1.so)
==8818==    by 0x401197: main::_e915de59229b3560 (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1964.stage1-x86_64-unknown-linux-gnu)
==8818==    by 0x4012F1: _rust_main (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1964.stage1-x86_64-unknown-linux-gnu)
==8818==    by 0x5754F5A: task_start_wrapper(spawn_args*) (rust_task.cpp:184)
==8818==  Address 0x0 is not stack'd, malloc'd or (recently) free'd
==8818==
==8818==
==8818== Process terminating with default action of signal 11 (SIGSEGV)
==8818==  Access not within mapped region at address 0x0
==8818==    at 0x4E7AC3C: str::hash::_31827ae260e7bf7b (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-14bd852465126fe7-0.1.so)
==8818==    by 0x51F68C4: map::chained::insert::_6cad5a231a3253ac (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-79ca5fac56b63fde-0.1.so)
==8818==    by 0x519FCC5: _ZN3map7chained7hashmap5168726insert17_76e8354b7f8b17bdE (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-79ca5fac56b63fde-0.1.so)
==8818==    by 0x401197: main::_e915de59229b3560 (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1964.stage1-x86_64-unknown-linux-gnu)
==8818==    by 0x4012F1: _rust_main (in /home/banderson/Dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/issue-1964.stage1-x86_64-unknown-linux-gnu)
==8818==    by 0x5754F5A: task_start_wrapper(spawn_args*) (rust_task.cpp:184)
==8818==  If you believe this happened as a result of a stack
==8818==  overflow in your program's main thread (unlikely but
==8818==  possible), you can try to increase the size of the
==8818==  main thread stack using the --main-stacksize= flag.
==8818==  The main thread stack size used in this run was 16777216.
