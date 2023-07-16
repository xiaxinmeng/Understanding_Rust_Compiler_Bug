
$ touch foo
$ gdb --args x86_64-unknown-linux-gnu/stage2/bin/rustc -Z ls foo
GNU gdb (Ubuntu 7.7-0ubuntu3.1) 7.7
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from x86_64-unknown-linux-gnu/stage2/bin/rustc...(no debugging symbols found)...done.
(gdb) r
Starting program: /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/rustc -Z ls foo
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[New Thread 0x7fffeffff480 (LWP 18508)]

Program received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffeffff480 (LWP 18508)]
0x00007ffff290977b in LLVMGetSections () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/./librustc_llvm-4e7c5e5c.so
(gdb) bt
#0  0x00007ffff290977b in LLVMGetSections () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/./librustc_llvm-4e7c5e5c.so
#1  0x00007ffff1d10061 in mk_section_iter::h017761c80330139dsxc () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/./librustc_llvm-4e7c5e5c.so
#2  0x00007ffff70c3239 in metadata::loader::get_metadata_section::hdc951672823ca586pgx () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#3  0x00007ffff70c49f1 in metadata::loader::list_file_metadata::h81b70bb085298e94rFx () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#4  0x00007ffff715da50 in driver::run_compiler::h4f9587d3bfd9e143JIB () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#5  0x00007ffff715c0b5 in driver::main_args::closure.138206 () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#6  0x00007ffff716e208 in task::TaskBuilder$LT$S$GT$::try_future::closure.139322 () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#7  0x00007ffff716e123 in task::TaskBuilder$LT$S$GT$::spawn_internal::closure.139299 () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc-4e7c5e5c.so
#8  0x00007ffff7ba85f8 in task::spawn_opts::closure.8367 () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/libnative-4e7c5e5c.so
#9  0x00007ffff64f798c in rust_try_inner () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#10 0x00007ffff64f7976 in rust_try () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#11 0x00007ffff649c103 in unwind::try::h7fcc2343690b9aebf7d () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#12 0x00007ffff649beff in task::Task::run::hdcdde8a1f17a9354zdd () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#13 0x00007ffff7ba8442 in task::spawn_opts::closure.8313 () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/libnative-4e7c5e5c.so
#14 0x00007ffff649dc9f in thread::thread_start::haf9f3cdef1730f4alCd () from /home/alex/code/rust2/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustrt-4e7c5e5c.so
#15 0x00007ffff574c182 in start_thread (arg=0x7fffeffff480) at pthread_create.c:312
#16 0x00007ffff616538d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111
