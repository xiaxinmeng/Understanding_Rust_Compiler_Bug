
==22826== Thread 3:
==22826== Use of uninitialised value of size 8
==22826==    at 0x401237: upd::_40f19aefb9a467f6 (in /home/brian/dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/test.stage1-x86_64-unknown-linux-gnu)
==22826==    by 0x4013C2: main::_843f8e32b1779b32 (in /home/brian/dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/test.stage1-x86_64-unknown-linux-gnu)
==22826==    by 0x401599: _rust_main (in /home/brian/dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/test.stage1-x86_64-unknown-linux-gnu)
==22826==    by 0x5156395: task_start_wrapper(spawn_args*) (rust_task.cpp:181)
==22826==
==22826==
==22826== Process terminating with default action of signal 11 (SIGSEGV)
==22826==  Bad permissions for mapped region at address 0x4A2366C
==22826==    at 0x401241: upd::_40f19aefb9a467f6 (in /home/brian/dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/test.stage1-x86_64-unknown-linux-gnu)
==22826==    by 0x4013C2: main::_843f8e32b1779b32 (in /home/brian/dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/test.stage1-x86_64-unknown-linux-gnu)
==22826==    by 0x401599: _rust_main (in /home/brian/dev/rust/build/x86_64-unknown-linux-gnu/test/run-pass/test.stage1-x86_64-unknown-linux-gnu)
==22826==    by 0x5156395: task_start_wrapper(spawn_args*) (rust_task.cpp:181)
