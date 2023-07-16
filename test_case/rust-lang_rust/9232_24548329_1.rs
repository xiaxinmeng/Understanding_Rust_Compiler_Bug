
[jdm@rosencrantz build]$ rustc /tmp/a.rs -Z debug-info
(gdb) break rust_begin_unwind
Function "rust_begin_unwind" not defined.
Make breakpoint pending on future shared library load? (y or [n]) y
Breakpoint 1 (rust_begin_unwind) pending.
(gdb) r
Starting program: /tmp/a 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".
[New Thread 0x7ffff75c0700 (LWP 30946)]
[New Thread 0x7ffff6bff700 (LWP 30947)]
task <unnamed> failed at 'whoops', /tmp/a.rs:2
[Switching to Thread 0x7ffff75c0700 (LWP 30946)]

Breakpoint 1, rust_begin_unwind (token=839147) at /home/jdm/sdb/rust/src/rt/rust_builtin.cpp:532
532 rust_begin_unwind(uintptr_t token) {
Missing separate debuginfos, use: debuginfo-install glibc-2.16-34.fc18.x86_64 libgcc-4.7.2-8.fc18.x86_64 libstdc++-4.7.2-8.fc18.x86_64
(gdb) bt
#0  rust_begin_unwind (token=839147) at /home/jdm/sdb/rust/src/rt/rust_builtin.cpp:532
#1  0x00007ffff797a9c6 in rt::task::Unwinder::begin_unwind::h7c12263797ed0787oaP::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#2  0x00007ffff7979e6f in sys::begin_unwind_::h89e154cd0915671::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#3  0x00007ffff7979fc2 in sys::__extensions__::fail_with::anon::anon::expr_fn::aF () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#4  0x00007ffff7979978 in c_str::ToCStr::with_c_str::hc6798931b183a7aA::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#5  0x00007ffff7979f71 in sys::__extensions__::fail_with::anon::expr_fn::aE () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#6  0x00007ffff7979978 in c_str::ToCStr::with_c_str::hc6798931b183a7aA::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#7  0x00007ffff790297c in sys::FailWithCause$__extensions__::fail_with::hdb4c44d01ce4116qua8::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#8  0x0000000000400a61 in a::foo () at /tmp/a.rs:48
#9  0x0000000000400aa9 in a::main () at /tmp/a.rs:6
#10 0x0000000000400ade in _rust_main ()
#11 0x00007ffff7997dc0 in rt::task::__extensions__::build_start_wrapper::anon::anon::expr_fn::aL () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#12 0x00007ffff79962b7 in rt::task::__extensions__::run::anon::expr_fn::a3 () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#13 0x00007ffff75fc473 in rust_try (f=<optimized out>, fptr=<optimized out>, env=<optimized out>) at /home/jdm/sdb/rust/src/rt/rust_builtin.cpp:523
#14 0x00007ffff79961ec in rt::task::Unwinder::try::h199ab8d6eb2269807oa2::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#15 0x00007ffff7996061 in rt::task::Task::run::h199ab8d6eb226980r2a1::v0.8$x2dpre () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#16 0x00007ffff79979ac in rt::task::__extensions__::build_start_wrapper::anon::expr_fn::aC () from /tmp/../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
#17 0x0000000000000000 in ?? ()
(gdb) fr 9
#9  0x0000000000400aa9 in a::main () at /tmp/a.rs:6
6     foo();
(gdb) dow
#8  0x0000000000400a61 in a::foo () at /tmp/a.rs:48
(gdb) 
