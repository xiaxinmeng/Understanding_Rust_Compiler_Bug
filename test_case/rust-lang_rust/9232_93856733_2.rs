
$ rust-gdb hello
<snip>
Reading symbols from hello...done.
(gdb) break rust_panic
Breakpoint 2 at 0x555555560e50
(gdb) r
Starting program: /home/steve/tmp/hello 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
thread '<main>' panicked at 'whoops', hello.rs:2

Breakpoint 2, 0x0000555555560e50 in rust_panic ()
(gdb) stacktrace
Undefined command: "stacktrace".  Try "help".
(gdb) st
Ambiguous command "st": stack, start, status, step, stepi, stepping, stop, strace.
(gdb) bt
#0  0x0000555555560e50 in rust_panic ()
#1  0x000055555555aeeb in rt::unwind::begin_unwind_inner::h082f07869f4b9731ACI ()
#2  0x000055555555a2cf in hello::rt::unwind::begin_unwind<&str> (msg="whoops", 
    file_line=0x5555557a96d0 <foo::_FILE_LINE::h7e3aee1d68787d09qaa>)
    at /home/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libstd/rt/unwind.rs:524
#3  0x000055555555a21b in hello::foo () at <std macros>:3
#4  0x000055555555a61e in hello::main () at hello.rs:6
#5  0x0000555555565409 in rust_try_inner ()
#6  0x00005555555653f6 in rust_try ()
#7  0x00005555555632cf in rt::lang_start::he050f8de3bcc02b7VRI ()
#8  0x000055555555a765 in main ()
(gdb) 
