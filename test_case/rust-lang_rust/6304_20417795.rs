
#0  upcall_fail (expr=0x7fffe48c5de0 "assertion failed: region_maps.scopes_intersect(old_loan.kill_scope, new_loan.kill_scope)", 
    file=0x7ffff6877450 <str67452> "/home/cmr/hacking/rust/src/librustc/middle/borrowck/check_loans.rs", line=181) at ../src/rt/rust_upcall.cpp:127
#1  0x00007ffff78c1eac in sys::begin_unwind_::_89e154cd0915671::_07 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#2  0x00007ffff78c1ad2 in sys::__extensions__::fail_with::anon::anon::expr_fn_21919 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#3  0x00007ffff78c1a01 in str::as_buf_21914::_f8c6f4a6f9cb738::_07 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#4  0x00007ffff78c1a8c in sys::__extensions__::fail_with::anon::expr_fn_21917 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#5  0x00007ffff78c1a01 in str::as_buf_21914::_f8c6f4a6f9cb738::_07 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#6  0x00007ffff782bf1b in sys::__extensions__::meth_10068::fail_with::_d96679812a86c367::_07 ()
   from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#7  0x00007ffff6610e8c in middle::borrowck::check_loans::__extensions__::meth_67539::report_error_if_loans_conflict::_7a7bbb95dbe43ed::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#8  0x00007ffff661087a in middle::borrowck::check_loans::__extensions__::check_for_conflicting_loans::anon::expr_fn_67532 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#9  0x00007ffff660ef92 in middle::borrowck::check_loans::__extensions__::meth_67420::each_issued_loan::_a8f64f5878cbbed::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#10 0x00007ffff661022b in middle::borrowck::check_loans::__extensions__::meth_67513::check_for_conflicting_loans::_da3080ceacf4c36f::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#11 0x00007ffff660ba89 in middle::borrowck::check_loans::check_loans_in_expr::_c19a735d2f695f4::_07 ()
---Type <return> to continue, or q <return> to quit---
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#12 0x00007ffff6239cea in visit::visit_expr_27456::_c1574f318f1f1a6::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#13 0x00007ffff660b7c9 in middle::borrowck::check_loans::check_loans_in_expr::_c19a735d2f695f4::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#14 0x00007ffff623a4d4 in visit::visit_exprs_27459::_b96d71a127b74940::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#15 0x00007ffff6238eb4 in visit::visit_expr_27456::_c1574f318f1f1a6::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#16 0x00007ffff660b7c9 in middle::borrowck::check_loans::check_loans_in_expr::_c19a735d2f695f4::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#17 0x00007ffff6239961 in visit::visit_expr_27456::_c1574f318f1f1a6::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#18 0x00007ffff660b7c9 in middle::borrowck::check_loans::check_loans_in_expr::_c19a735d2f695f4::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#19 0x00007ffff6234adb in visit::visit_expr_opt_27065::_719f6211725a297::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#20 0x00007ffff6233fb4 in visit::visit_block_26910::_2945d7bc2771dfd::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#21 0x00007ffff660df58 in middle::borrowck::check_loans::check_loans_in_block::_535c1a3589f6e3::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#22 0x00007ffff623954a in visit::visit_expr_27456::_c1574f318f1f1a6::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#23 0x00007ffff660b7c9 in middle::borrowck::check_loans::check_loans_in_expr::_c19a735d2f695f4::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
---Type <return> to continue, or q <return> to quit---
#24 0x00007ffff6234adb in visit::visit_expr_opt_27065::_719f6211725a297::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#25 0x00007ffff6233fb4 in visit::visit_block_26910::_2945d7bc2771dfd::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#26 0x00007ffff660df58 in middle::borrowck::check_loans::check_loans_in_block::_535c1a3589f6e3::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#27 0x00007ffff6240023 in visit::visit_arm_27993::_fe1ea04ccc534::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#28 0x00007ffff623fe6c in visit::default_visitor_27605::anon::expr_fn_27991 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#29 0x00007ffff6239464 in visit::visit_expr_27456::_c1574f318f1f1a6::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#30 0x00007ffff660b7c9 in middle::borrowck::check_loans::check_loans_in_expr::_c19a735d2f695f4::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#31 0x00007ffff6234adb in visit::visit_expr_opt_27065::_719f6211725a297::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#32 0x00007ffff6233fb4 in visit::visit_block_26910::_2945d7bc2771dfd::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#33 0x00007ffff660df58 in middle::borrowck::check_loans::check_loans_in_block::_535c1a3589f6e3::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#34 0x00007ffff660035d in middle::borrowck::check_loans::check_loans::_78399e2f71ef86d::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#35 0x00007ffff665a0de in middle::borrowck::borrowck_fn::_b7106e69c6c155e3::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#36 0x00007ffff623f465 in visit::visit_method_helper_27933::_ccd15c8939a283f7::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#37 0x00007ffff623f164 in visit::visit_item_27804::_72d5616277254d51::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
---Type <return> to continue, or q <return> to quit---
#38 0x00007ffff623e11c in visit::default_visitor_27605::anon::expr_fn_27802 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#39 0x00007ffff623d913 in visit::visit_mod_27738::_180342e1b369096::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#40 0x00007ffff623d748 in visit::default_visitor_27605::anon::expr_fn_27736 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#41 0x00007ffff65f0f1a in visit::visit_crate_65814::_85b5b27c66c5cece::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#42 0x00007ffff6655025 in middle::borrowck::check_crate::_66a2ce23a87e3a87::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#43 0x00007ffff682f9c1 in driver::driver::compile_rest::anon::expr_fn_93289 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#44 0x00007ffff682d083 in driver::driver::compile_rest::_e58f61c4387ba8b7::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#45 0x00007ffff6861854 in __morestack () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#46 0x00007ffff682ffbc in driver::driver::compile_upto::_af4a64d82884294::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#47 0x00007ffff6830355 in driver::driver::compile_input::_2d802f44a752fd9a::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#48 0x00007ffff684f793 in run_compiler::_7449a6c8ce585463::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#49 0x00007ffff686157e in main::anon::expr_fn_97271 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#50 0x00007ffff685f027 in monitor::anon::expr_fn_97096 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#51 0x00007ffff685693b in task::__extensions__::try_95957::anon::expr_fn_96395 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#52 0x00007ffff6861854 in __morestack () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#53 0x00007ffff7894961 in task::spawn::spawn_raw_oldsched::make_child_wrapper::anon::expr_fn_18373 ()
   from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#54 0x00007ffff790ad7c in __morestack () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
---Type <return> to continue, or q <return> to quit---
#55 0x00007ffff5e54e43 in task_start_wrapper (a=0x7fffec20dcb0) at ../src/rt/rust_task.cpp:164
#56 0x0000000000000000 in ?? ()
