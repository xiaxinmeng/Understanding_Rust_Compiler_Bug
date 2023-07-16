
#0  0x00007ffff471dd67 in raise () from /usr/lib/libc.so.6
#1  0x00007ffff471f118 in abort () from /usr/lib/libc.so.6
#2  0x00007ffff4716bdd in __assert_fail_base () from /usr/lib/libc.so.6
#3  0x00007ffff4716c92 in __assert_fail () from /usr/lib/libc.so.6
#4  0x00007ffff68aeb7e in llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&) ()
   from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#5  0x00007ffff5ba4a28 in llvm::IRBuilder<true, llvm::ConstantFolder, llvm::IRBuilderDefaultInserter<true> >::CreateCall(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&) () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#6  0x00007ffff680eea7 in LLVMBuildCall () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#7  0x00007ffff5227c49 in middle::trans::builder::Builder$LT$$x27a$GT$::call::h62f30ad143616955dlo::v0.11.0.pre ()
   from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#8  0x00007ffff519b0b6 in middle::trans::base::invoke::hf181fbfee0b1ec67Vyp::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#9  0x00007ffff51b4d3c in middle::trans::callee::trans_call_inner::hc36896ed5884e7b3xTe::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#10 0x00007ffff51c178d in middle::trans::expr::trans_rvalue_dps_unadjusted::hf8c3a53b294796catmg::v0.11.0.pre ()
   from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#11 0x00007ffff517d0c1 in middle::trans::expr::trans_into::h8e2421ad39604664ovf::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#12 0x00007ffff517c1c9 in middle::trans::controlflow::trans_stmt_semi::h6e7e9d174b6b26427hc::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#13 0x00007ffff517b8a8 in middle::trans::controlflow::trans_stmt::hd21602ca8e28676aKdc::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#14 0x00007ffff517d421 in middle::trans::controlflow::trans_block::hf672839f6c7e9c080ic::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#15 0x00007ffff524260b in middle::trans::base::trans_closure::h17099e5455a18d78Wjq::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#16 0x00007ffff514c3d3 in middle::trans::base::trans_fn::h4d41312aeb1f9eefXrq::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#17 0x00007ffff514502c in middle::trans::base::trans_item::h85d366f03e478cc8mIq::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#18 0x00007ffff5253311 in middle::trans::base::trans_crate::h3f81b45a8d41b127TBr::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#19 0x00007ffff5a81830 in driver::driver::phase_4_translate_to_llvm::h20c9996d1965b7955yv::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#20 0x00007ffff5a76ba6 in driver::driver::compile_input::h3da42a086c5ec4b59dv::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#21 0x00007ffff5b42937 in driver::run_compiler::h36852bed2c8a2fe6LXx::v0.11.0.pre () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#22 0x00007ffff5b3ff85 in driver::main_args::closure.98559 () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#23 0x00007ffff5b5bb3b in driver::monitor::closure.99649 () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#24 0x00007ffff5b569f8 in task::TaskBuilder::try::closure.99412 () from /usr/local/bin/../lib/librustc-d252d482-0.11.0-pre.so
#25 0x00007ffff7baa288 in task::spawn_opts::closure.7149 () from /usr/local/bin/../lib/libnative-1fb5e2c0-0.11.0-pre.so
#26 0x00007ffff4afbfe3 in task::Task::run::closure.5308 () from /usr/local/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#27 0x00007ffff4b614cc in rust_try () from /usr/local/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#28 0x00007ffff4afea36 in unwind::try::hff9f11c8495162d3fGd::v0.11.0.pre () from /usr/local/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#29 0x00007ffff4afbe95 in task::Task::run::hd14e65be9d626e23VVc::v0.11.0.pre () from /usr/local/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#30 0x00007ffff7baa0fb in task::spawn_opts::closure.7122 () from /usr/local/bin/../lib/libnative-1fb5e2c0-0.11.0-pre.so
#31 0x00007ffff4afdfe4 in thread::thread_start::h29b39a0f53979794kdd::v0.11.0.pre () from /usr/local/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#32 0x00007ffff3dd5124 in start_thread () from /usr/lib/libpthread.so.0
#33 0x00007ffff47d34bd in clone () from /usr/lib/libc.so.6
