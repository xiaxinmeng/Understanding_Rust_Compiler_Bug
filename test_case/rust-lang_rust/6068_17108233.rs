
#0  0x00007ff3fed321c9 in raise () from /usr/lib/libc.so.6
#1  0x00007ff3fed335c8 in abort () from /usr/lib/libc.so.6
#2  0x00007ff3fed2b356 in __assert_fail_base () from /usr/lib/libc.so.6
#3  0x00007ff3fed2b402 in __assert_fail () from /usr/lib/libc.so.6
#4  0x00007ff400961d06 in llvm::CastInst::CreatePointerCast(llvm::Value*, llvm::Type*, llvm::Twine const&, llvm::Instruction*) () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustllvm.so
#5  0x00007ff4008e4329 in LLVMBuildPointerCast () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustllvm.so
#6  0x00007ff401baeac2 in LLVMBuildPointerCast__c_stack_shim () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#7  0x00007ff3ff9f99f1 in __morestack () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustrt.so
#8  0x00007ff3ff9ebe8f in call_on_c_stack (fn_ptr=<optimized out>, args=0x7ff3f021fde0, this=0x7ff3f8208720) at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_task.h:470
#9  upcall_call_shim_on_c_stack (args=0x7ff3f021fde0, fn_ptr=<optimized out>) at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_upcall.cpp:64
#10 0x00007ff4014bf213 in middle::trans::build::PointerCast::_b4c7e36e9132cf2a::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#11 0x00007ff4014ff0cd in middle::trans::callee::trans_call_inner::anon::expr_fn_26930 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#12 0x00007ff4014bc0e2 in middle::trans::base::with_scope::_8e8877abebb889d7::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#13 0x00007ff4014ef182 in middle::trans::callee::trans_call_inner::_45e9c4d4a24dfcd::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#14 0x00007ff4014ef4aa in middle::trans::callee::trans_method_call::_75d9989f88bdf3::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#15 0x00007ff40150c1ba in middle::trans::expr::trans_rvalue_dps_unadjusted::_79fd6262e0af7182::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#16 0x00007ff40146e627 in middle::trans::expr::trans_into::_79fd6262e0af7182::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#17 0x00007ff401bc8e84 in __morestack () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#18 0x00007ff40146ab73 in middle::trans::controlflow::trans_block::_392a2863987a5dc::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#19 0x00007ff40157c6ea in middle::trans::base::trans_closure::_f12f8f9eb03ea12c::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#20 0x00007ff401424339 in middle::trans::base::trans_fn::_42ce2c995a187af::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#21 0x00007ff40141c026 in middle::trans::base::trans_item::_d3d87556b86b3cc0::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#22 0x00007ff401583043 in middle::trans::base::trans_mod::_ea48e27a14d205c::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#23 0x00007ff40141b69e in middle::trans::base::trans_item::_d3d87556b86b3cc0::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#24 0x00007ff401583043 in middle::trans::base::trans_mod::_ea48e27a14d205c::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#25 0x00007ff4015a2af4 in middle::trans::base::trans_crate::_3af69699ac82ea9::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#26 0x00007ff401b8a4a0 in driver::driver::compile_rest::anon::expr_fn_79892 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#27 0x00007ff401b874ee in driver::driver::compile_rest::_87c0f340aa21a39::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#28 0x00007ff401bc8e84 in __morestack () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#29 0x00007ff401b8a738 in driver::driver::compile_upto::_84743710faafb0af::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#30 0x00007ff401bc8e84 in __morestack () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#31 0x00007ff401b8ab7a in driver::driver::compile_input::_a6b0fe22676e6b::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#32 0x00007ff401bba31a in run_compiler::_9de519bbeb75837::_06 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#33 0x00007ff401bc6951 in monitor::anon::expr_fn_81957 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#34 0x00007ff401bc317c in task::__extensions__::try_81478::anon::expr_fn_81741 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#35 0x00007ff401bc8e84 in __morestack () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc-c84825241471686d-0.6.so
#36 0x00007ff402bdb38e in task::spawn::spawn_raw::make_child_wrapper::anon::expr_fn_12071 () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#37 0x00007ff402c5b214 in __morestack () from /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/libcore-c3ca5d77d81b46c1-0.6.so
#38 0x00007ff3ff9ea534 in task_start_wrapper (a=<optimized out>) at /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/rt/rust_task.cpp:162
#39 0x0000000000000000 in ?? ()
