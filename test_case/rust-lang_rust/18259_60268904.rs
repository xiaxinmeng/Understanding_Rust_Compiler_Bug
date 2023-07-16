
$ gdb --args ./x86_64-unknown-linux-gnu/stage1/bin/rustc src/libcore/lib.rs
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
Reading symbols from ./x86_64-unknown-linux-gnu/stage1/bin/rustc...(no debugging symbols found)...done.
(gdb) r
Starting program: /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/rustc src/libcore/lib.rs
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[New Thread 0x7ffff2bff480 (LWP 18546)]

Program received signal SIGILL, Illegal instruction.
[Switching to Thread 0x7ffff2bff480 (LWP 18546)]
0x00007ffff7e84278 in bitv::Bitv::set::h693713f7b138c463Yla () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-4e7c5e5c.so
(gdb) bt
#0  0x00007ffff7e84278 in bitv::Bitv::set::h693713f7b138c463Yla () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-4e7c5e5c.so
#1  0x00007ffff7e88574 in bitv::BitvSet.MutableSet$LT$uint$GT$::insert::h437317dbaeae41b5i8a () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-4e7c5e5c.so
#2  0x00007ffff4313f72 in attr::first_attr_value_str_by_name::h23b9b3a677724c717XC () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-4e7c5e5c.so
#3  0x00007ffff434d0bc in parse::parser::Parser$LT$$x27a$GT$::eval_src_mod::hac338816627dce54NaQ () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-4e7c5e5c.so
#4  0x00007ffff433ee8e in parse::parser::Parser$LT$$x27a$GT$::parse_item_or_view_item::h56365d466ced5143sEQ ()
   from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-4e7c5e5c.so
#5  0x00007ffff4346846 in parse::parser::Parser$LT$$x27a$GT$::parse_items_and_view_items::h6c430a0fa80b713eT6Q ()
   from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-4e7c5e5c.so
#6  0x00007ffff434ae70 in parse::parser::Parser$LT$$x27a$GT$::parse_mod_items::hf066dcd3ed21aeb821P () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-4e7c5e5c.so
#7  0x00007ffff43549ef in parse::parser::Parser$LT$$x27a$GT$::parse_crate_mod::h4947cca241ec7fec9bR () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-4e7c5e5c.so
#8  0x00007ffff43acabd in parse::parse_crate_from_file::h100116cb38a295e7NsV () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-4e7c5e5c.so
#9  0x00007ffff71658f0 in driver::driver::phase_1_parse_input::closure.142527 () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-4e7c5e5c.so
#10 0x00007ffff715c5a2 in driver::driver::phase_1_parse_input::h4f0913c2647baa03ibA () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-4e7c5e5c.so
#11 0x00007ffff7159feb in driver::driver::compile_input::hd529ab663911c532g6z () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-4e7c5e5c.so
#12 0x00007ffff71b99cf in driver::run::closure.147104 () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-4e7c5e5c.so
#13 0x00007ffff71c4327 in task::TaskBuilder$LT$S$GT$::try_future::closure.147673 () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-4e7c5e5c.so
#14 0x00007ffff71c42d8 in task::TaskBuilder$LT$S$GT$::spawn_internal::closure.147667 () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-4e7c5e5c.so
#15 0x00007ffff7fc287f in task::NativeSpawner.Spawner::spawn::closure.8465 () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/libnative-4e7c5e5c.so
#16 0x00007ffff7edd6ec in rust_try_inner () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-4e7c5e5c.so
#17 0x00007ffff7edd6d6 in rust_try () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-4e7c5e5c.so
#18 0x00007ffff7e7d0c3 in unwind::try::hc8101bb7ca759647imd () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-4e7c5e5c.so
#19 0x00007ffff7e7ed5c in task::Task::run::hb7a787d36e5bed95WBc () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-4e7c5e5c.so
#20 0x00007ffff7fc26f8 in task::NativeSpawner.Spawner::spawn::closure.8406 () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/libnative-4e7c5e5c.so
#21 0x00007ffff7e7cec8 in thread::thread_start::h9a3d162da155dd197Vc () from /home/alex/code/rust3/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-4e7c5e5c.so
#22 0x00007ffff6633182 in start_thread (arg=0x7ffff2bff480) at pthread_create.c:312
#23 0x00007ffff6bdffbd in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:111
