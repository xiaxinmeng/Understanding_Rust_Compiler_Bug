

#0  rust_task_fail (task=0x7fffec203c80, 
    expr=0x7fffe45cb140 "index out of bounds: the len is 0 but the index is 0", file=0x7ffff62ae1a0 <str65739> "/home/ermolovd/workspace/rust/src/librustc/middle/check_match.rs", line=231)
    at /home/ermolovd/workspace/rust/src/rt/rust_task.cpp:90
#1  0x00007ffff50a0d59 in __morestack ()
   from /home/ermolovd/.local/bin/../lib/librustrt.so
#2  0x00007ffff50917b8 in call_on_c_stack (
    fn_ptr=0x7ffff50910f0 <upcall_s_fail(s_fail_args*)>, args=0x7ffff15f1360, 
    this=0x7fffec203c80)
    at /home/ermolovd/workspace/rust/src/rt/rust_task.h:481
#3  call_upcall_on_c_stack (
    fn_ptr=0x7ffff50910f0 <upcall_s_fail(s_fail_args*)>, args=0x7ffff15f1360, 
    task=<optimized out>)
    at /home/ermolovd/workspace/rust/src/rt/rust_upcall.cpp:51
#4  upcall_fail (expr=<optimized out>, 
    file=0x7ffff62ae1a0 <str65739> "/home/ermolovd/workspace/rust/src/librustc/middle/check_match.rs", line=231)
    at /home/ermolovd/workspace/rust/src/rt/rust_upcall.cpp:134
#5  0x00007ffff77d88d7 in sys::rustrt::rust_upcall_fail::_f25e955198ccc5c0::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#6  0x00007ffff77d8e61 in sys::begin_unwind_::_89e154cd0915671::_0$x2e8$x2dpre
---Type <return> to continue, or q <return> to quit---
    () from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#7  0x00007ffff76b2a0e in unstable::lang::fail_::_89e154cd0915671::_0$x2e8$x2dpre () from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#8  0x00007ffff77ed66a in unstable::lang::fail_bounds_check::anon::expr_fn_26335 () from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#9  0x00007ffff779fe09 in str::__extensions__::as_c_str_23197::anon::expr_fn_23200 () from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#10 0x00007ffff770450a in vec::__extensions__::as_imm_buf_15036::_ad5b8a27a3c3790::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#11 0x00007ffff773274d in str::__extensions__::as_imm_buf_17989::_ad5b8a27a3c3790::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#12 0x00007ffff779fbc0 in str::__extensions__::as_c_str_23197::_d69de411ee2549::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#13 0x00007ffff76bf4e1 in unstable::lang::fail_bounds_check::_7112ff25e39642a2::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#14 0x00007ffff5ca4467 in middle::check_match::is_useful::_1efec9c180b8af3::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#15 0x00007ffff5cac580 in middle::check_match::is_useful_specialized::_a8ff93c88---Type <return> to continue, or q <return> to quit---
f76c279::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#16 0x00007ffff5ca63bf in middle::check_match::is_useful::_1efec9c180b8af3::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#17 0x00007ffff5c9e39b in middle::check_match::check_arms::_1c832619b9e2589c::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#18 0x00007ffff5c9b75b in middle::check_match::check_expr::_7a4496e893c2d5d1::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#19 0x00007ffff5c9afeb in middle::check_match::check_crate::anon::expr_fn_65576
    ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#20 0x00007ffff5945939 in oldvisit::visit_expr_opt_51892::_18f64827e554d7cd::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#21 0x00007ffff59455a1 in oldvisit::visit_block_51889::_e6e619698347db51::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#22 0x00007ffff594523f in oldvisit::default_visitor_51839::anon::expr_fn_51887
    ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
---Type <return> to continue, or q <return> to quit---
#23 0x00007ffff5950db7 in oldvisit::visit_fn_51940::_d9546aecbdd6eee::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#24 0x00007ffff5c9d4c1 in middle::check_match::check_fn::_85bb11c86427ffcc::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#25 0x00007ffff5c9d28e in middle::check_match::check_crate::anon::expr_fn_65594
    ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#26 0x00007ffff5942694 in oldvisit::visit_item_51864::_b83b1cc7f1a7ba5b::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#27 0x00007ffff5941a23 in oldvisit::default_visitor_51839::anon::expr_fn_51862
    ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#28 0x00007ffff5940cf7 in oldvisit::visit_mod_51844::_6976f7d8c85555ba::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#29 0x00007ffff5940884 in oldvisit::default_visitor_51839::anon::expr_fn_51842
    ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#30 0x00007ffff56e82be in oldvisit::visit_crate_37522::_f46a3e54d45a1f4::_0$x2e8$x2dpre ()
---Type <return> to continue, or q <return> to quit---
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#31 0x00007ffff5c9a81a in middle::check_match::check_crate::_3edff8522e9cc9ee::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#32 0x00007ffff621d836 in driver::driver::phase_3_run_analysis_passes::anon::expr_fn_95862 ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#33 0x00007ffff5c7f5d9 in util::common::time_65127::_c4d0513e54dc658e::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#34 0x00007ffff621ac5b in driver::driver::phase_3_run_analysis_passes::_1b99c65643aa108b::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#35 0x00007ffff6220904 in driver::driver::compile_input::_4c53e9c6f69a01::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#36 0x00007ffff625c637 in run_compiler::_0184613adfc5db9::_0$x2e8$x2dpre ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#37 0x00007ffff62984ea in main::anon::expr_fn_101643 ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#38 0x00007ffff629248c in monitor::anon::expr_fn_101401 ()
   from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#39 0x00007ffff627fe06 in task::__extensions__::try_100268::anon::expr_fn_100686---Type <return> to continue, or q <return> to quit---
 () from /home/ermolovd/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#40 0x00007ffff778baf7 in task::spawn::spawn_raw_oldsched::make_child_wrapper::anon::expr_fn_22464 ()
   from /home/ermolovd/.local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#41 0x00007ffff50900fb in task_start_wrapper (a=0x7ffff15f8820)
    at /home/ermolovd/workspace/rust/src/rt/rust_task.cpp:162
#42 0x0000000000000000 in ?? ()

