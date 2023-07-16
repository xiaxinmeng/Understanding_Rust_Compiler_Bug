
Breakpoint 1, rust_task_fail (task=0x7ffff0204790, expr=0x7fffe84a41a0 "assertion failed: rp.is_none()", file=0x7ffff7114570 <str59342> "/home/jdm/sdb/rust/src/librustc/middle/typeck/collect.rs", line=1066)
    at /home/jdm/sdb/rust/src/rt/rust_task.cpp:91
warning: Source file is more recent than executable.
91      task->begin_failure(expr, file, line);
Missing separate debuginfos, use: debuginfo-install glibc-2.16-33.fc18.x86_64 libgcc-4.7.2-8.fc18.x86_64 libstdc++-4.7.2-8.fc18.x86_64
(gdb) bt
#0  rust_task_fail (task=0x7ffff0204790, expr=0x7fffe84a41a0 "assertion failed: rp.is_none()", file=0x7ffff7114570 <str59342> "/home/jdm/sdb/rust/src/librustc/middle/typeck/collect.rs", line=1066)
    at /home/jdm/sdb/rust/src/rt/rust_task.cpp:91
#1  0x00007ffff65907f9 in __morestack () from /usr/local/bin/../lib/librustrt.so
#2  0x00007ffff6581aa0 in call_on_c_stack (fn_ptr=0x7ffff65813e0 <upcall_s_fail(s_fail_args*)>, args=0x7ffff46c34f0, this=0x7ffff0204790) at /home/jdm/sdb/rust/src/rt/rust_task.h:481
#3  call_upcall_on_c_stack (fn_ptr=0x7ffff65813e0 <upcall_s_fail(s_fail_args*)>, args=0x7ffff46c34f0, task=<optimized out>) at /home/jdm/sdb/rust/src/rt/rust_upcall.cpp:51
#4  upcall_fail (expr=<optimized out>, file=0x7ffff7114570 <str59342> "/home/jdm/sdb/rust/src/librustc/middle/typeck/collect.rs", line=1066) at /home/jdm/sdb/rust/src/rt/rust_upcall.cpp:134
#5  0x00007ffff7d29103 in sys::begin_unwind_::_615cb041c655a97::_07pre () from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#6  0x00007ffff7d29092 in sys::__extensions__::fail_with::anon::anon::expr_fn_20105 () from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#7  0x00007ffff7c6902b in sys::__extensions__::meth_9151::fail_with::_ac9eb45dc42bd690::_07pre () from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#8  0x00007ffff6c8bb0d in middle::typeck::collect::ty_of_item::_af587f345880ea15::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#9  0x00007ffff6c8698b in middle::typeck::collect::convert::_d0cba369a739ab4::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#10 0x00007ffff6c8636b in middle::typeck::collect::collect_item_types::anon::expr_fn_59405 () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#11 0x00007ffff7405ae6 in visit::mk_simple_visitor::anon::expr_fn_17109 () from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.7-pre.so
#12 0x00007ffff74050ca in visit::mk_simple_visitor::anon::expr_fn_17099 () from /usr/local/bin/../lib/libsyntax-84efebcb12c867a2-0.7-pre.so
#13 0x00007ffff6890ae1 in visit::visit_crate_34001::_a964efa1e9ab4f6::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#14 0x00007ffff6c8449e in middle::typeck::collect::collect_item_types::_3ba31b1c9b8b1891::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#15 0x00007ffff6cdedd2 in middle::typeck::check_crate::anon::expr_fn_60938 () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#16 0x00007ffff6cdec29 in util::common::time_60929::_a6aa6673bcdf8f66::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#17 0x00007ffff6cdc1f0 in middle::typeck::check_crate::_4387c6ff27b03d::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#18 0x00007ffff6fe349d in driver::driver::compile_rest::_134f5496ff5de1::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#19 0x00007ffff6fe9682 in driver::driver::compile_upto::_747e21c0469b22f2::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#20 0x00007ffff6fe9b07 in driver::driver::compile_input::_0b1e5b3afc4cd1::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#21 0x00007ffff701a32b in run_compiler::_63c0a0c62771367c::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#22 0x00007ffff702d00e in main::anon::expr_fn_89809 () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#23 0x00007ffff702a8fc in monitor::anon::expr_fn_89630 () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#24 0x00007ffff70242cc in task::__extensions__::try_89033::anon::expr_fn_89226 () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#25 0x00007ffff702d2e8 in __morestack () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#26 0x00007ffff7cf2045 in task::spawn::spawn_raw_oldsched::make_child_wrapper::anon::expr_fn_17074 () from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#27 0x00007ffff7d6e898 in __morestack () from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#28 0x00007ffff6580414 in task_start_wrapper (a=0x7ffff0205750) at /home/jdm/sdb/rust/src/rt/rust_task.cpp:167
#29 0x0000000000000000 in ?? ()
