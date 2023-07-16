
#0  rust_task_fail (task=0x7fffec208970, expr=0x7fffe473b4d0 "index out of bounds: the len is 1 but the index is 1", 
    file=0x7ffff6923c20 <str34878> "/home/siege/src/rust2/src/librustc/middle/trans/cabi_x86_64.rs", line=259)
    at /home/siege/src/rust2/src/rt/rust_task.cpp:86
#1  0x00007ffff5e0bb69 in __morestack () from /usr/local/bin/../lib/librustrt.so
#2  0x00007ffff5dfcb68 in call_on_c_stack (fn_ptr=0x7ffff5dfc4a0 <upcall_s_fail(s_fail_args*)>, args=0x7fffe420fe90, this=0x7fffec208970)
    at /home/siege/src/rust2/src/rt/rust_task.h:470
#3  call_upcall_on_c_stack (fn_ptr=0x7ffff5dfc4a0 <upcall_s_fail(s_fail_args*)>, args=0x7fffe420fe90, task=<optimized out>)
    at /home/siege/src/rust2/src/rt/rust_upcall.cpp:45
#4  upcall_fail (expr=<optimized out>, file=0x7ffff6923c20 <str34878> "/home/siege/src/rust2/src/librustc/middle/trans/cabi_x86_64.rs", 
    line=259) at /home/siege/src/rust2/src/rt/rust_upcall.cpp:128
#5  0x00007ffff79d011b in sys::begin_unwind_::_9873ff47b9982218::_07pre () from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#6  0x00007ffff791615e in unstable::lang::fail_::_9873ff47b9982218::_07pre ()
   from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#7  0x00007ffff791f4da in unstable::lang::fail_bounds_check::_6fb13a2fe60974b::_07pre ()
   from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#8  0x00007ffff6342c7d in middle::trans::cabi_x86_64::x86_64_tys::x86_64_ty::_cba3ba6a69dd212b::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#9  0x00007ffff6343544 in middle::trans::cabi_x86_64::x86_64_tys::anon::expr_fn_34991 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#10 0x00007ffff634374e in middle::trans::cabi_x86_64::__extensions__::meth_35015::compute_info::_63f5244ae590c1b6::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#11 0x00007ffff6346e3a in middle::trans::foreign::shim_types::_4ddc0485e8ab7a0::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#12 0x00007ffff634837d in middle::trans::foreign::trans_foreign_mod::build_foreign_fn::_cadbd7f24e409e5f::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#13 0x00007ffff6348060 in middle::trans::foreign::trans_foreign_mod::anon::expr_fn_35155 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#14 0x00007ffff62c146e in middle::trans::foreign::trans_foreign_mod::_f7e97aff93938699::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#15 0x00007ffff6155909 in middle::trans::base::trans_item::_e83b64764568c7be::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#16 0x00007ffff62c1253 in middle::trans::base::trans_mod::_d740d055c716f947::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#17 0x00007ffff62e1264 in middle::trans::base::trans_crate::_4f38ab124cab2e::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#18 0x00007ffff691ade4 in __morestack () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#19 0x00007ffff68d8c10 in driver::driver::compile_rest::anon::expr_fn_81655 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#20 0x00007ffff68d5c6a in driver::driver::compile_rest::_58068714615f2b::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#21 0x00007ffff691ade4 in __morestack () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#22 0x00007ffff68d8e28 in driver::driver::compile_upto::_3ccd839185751c4f::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#23 0x00007ffff68d926a in driver::driver::compile_input::_7bc345e59d22cba0::_07pre ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#24 0x00007ffff690b820 in run_compiler::_aa88c3dc1384374::_07pre () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#25 0x00007ffff6918791 in monitor::anon::expr_fn_83847 () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#26 0x00007ffff6914cba in task::__extensions__::try_83368::anon::expr_fn_83631 ()
   from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#27 0x00007ffff691ade4 in __morestack () from /usr/local/bin/../lib/librustc-c84825241471686d-0.7-pre.so
#28 0x00007ffff7997fbf in task::spawn::spawn_raw::make_child_wrapper::anon::expr_fn_12061 ()
   from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#29 0x00007ffff7a19264 in __morestack () from /usr/local/bin/../lib/libcore-c3ca5d77d81b46c1-0.7-pre.so
#30 0x00007ffff5dfb5c4 in task_start_wrapper (a=0x7fffec2097b0) at /home/siege/src/rust2/src/rt/rust_task.cpp:162
#31 0x0000000000000000 in ?? ()
