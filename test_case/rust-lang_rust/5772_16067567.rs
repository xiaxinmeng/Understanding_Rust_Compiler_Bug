
progval@Andromede:~/src/rust-0.6$ VERBOSE=1 RUST_LOG=rustc=1,::rt::backtrace make
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: using gcc
cfg: no pandoc found, omitting doc/rust.pdf
cfg: no llnextgen found, omitting grammar-verification
cfg: no pandoc found, omitting library doc build
x86_64-unknown-linux-gnu/stage0/bin/rustc --cfg stage0  -O  --target=x86_64-unknown-linux-gnu  -o x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so /home/progval/src/rust-0.6/src/librustc/rustc.rc && touch x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so
^Cmake: *** [x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so] Interrompre

progval@Andromede:~/src/rust-0.6$ gdb --args x86_64-unknown-linux-gnu/stage0/bin/rustc --cfg stage0  -O  --target=x86_64-unknown-linux-gnu  -o x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so /home/progval/src/rust-0.6/src/librustc/rustc.rc
GNU gdb (GDB) 7.4.1-debian
Copyright (C) 2012 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/rustc...(no debugging symbols found)...done.
(gdb) break rust_task_fail
Function "rust_task_fail" not defined.
Make breakpoint pending on future shared library load? (y or [n]) y
Breakpoint 1 (rust_task_fail) pending.
(gdb) run
Starting program: /home/progval/src/rust-0.6/x86_64-unknown-linux-gnu/stage0/bin/rustc --cfg stage0 -O --target=x86_64-unknown-linux-gnu -o x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so /home/progval/src/rust-0.6/src/librustc/rustc.rc
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[New Thread 0x7ffff7ff6700 (LWP 22059)]
[New Thread 0x7ffff7fdd700 (LWP 22063)]
[New Thread 0x7ffff7fd8700 (LWP 22064)]
[Switching to Thread 0x7ffff7fd8700 (LWP 22064)]

Breakpoint 1, rust_task_fail (task=0x6106f0, expr=0x26dd5c20 "explicit failure", file=0x20f0ba10 "/home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/libcore/run.rs", line=343)
    at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_task.cpp:85
85  /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_task.cpp: Aucun fichier ou dossier de ce type.
