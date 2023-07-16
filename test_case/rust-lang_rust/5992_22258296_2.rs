
rustc: /home/chris/vc/rust/src/llvm/lib/IR/Instructions.cpp:276: void llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&): Assertion `(Args.size() == FTy->getNumParams() || (FTy->isVarArg() && Args.size() > FTy->getNumParams())) && "Calling a function with bad signature!"' failed.

Program received signal SIGABRT, Aborted.
[Switching to Thread 0x7ffff7fc2700 (LWP 13558)]
0x00007ffff58dcf77 in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
56  ../nptl/sysdeps/unix/sysv/linux/raise.c: No such file or directory.
(gdb) bt
#0  0x00007ffff58dcf77 in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
#1  0x00007ffff58e05e8 in __GI_abort () at abort.c:90
#2  0x00007ffff58d5d43 in __assert_fail_base (fmt=0x7ffff5a2c3b8 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n", 
    assertion=assertion@entry=0x7ffff491a390 "(Args.size() == FTy->getNumParams() || (FTy->isVarArg() && Args.size() > FTy->getNumParams())) && \"Calling a function with bad signature!\"", 
    file=file@entry=0x7ffff4918768 "/home/chris/vc/rust/src/llvm/lib/IR/Instructions.cpp", line=line@entry=276, 
    function=function@entry=0x7ffff491b580 <llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&)::__PRETTY_FUNCTION__> "void llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&)") at assert.c:92
#3  0x00007ffff58d5df2 in __GI___assert_fail (
    assertion=0x7ffff491a390 "(Args.size() == FTy->getNumParams() || (FTy->isVarArg() && Args.size() > FTy->getNumParams())) && \"Calling a function with bad signature!\"", 
    file=0x7ffff4918768 "/home/chris/vc/rust/src/llvm/lib/IR/Instructions.cpp", line=276, 
    function=0x7ffff491b580 <llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&)::__PRETTY_FUNCTION__> "void llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&)") at assert.c:101
#4  0x00007ffff4264d9d in llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&) ()
   from /home/chris/opt/rust/bin/../lib/./librustllvm.so
#5  0x00007ffff3e81909 in llvm::IRBuilder<true, llvm::ConstantFolder, llvm::IRBuilderDefaultInserter<true> >::CreateCall(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&) ()
   from /home/chris/opt/rust/bin/../lib/./librustllvm.so
#6  0x00007ffff41e75c6 in LLVMBuildCall () from /home/chris/opt/rust/bin/../lib/./librustllvm.so
#7  0x00007ffff6129f27 in middle::trans::builder::__extensions__::meth_35620::call::_8fc96a176fc2e2::_0$x2e8$x2dpre () from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#8  0x00007ffff604fb4e in middle::trans::build::Call::_92a0f689105bc1f8::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#9  0x00007ffff60995d3 in middle::trans::base::invoke::_e1ab63192a1f6a6f::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#10 0x00007ffff609727d in middle::trans::callee::trans_call_inner::anon::expr_fn_29337 ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#11 0x00007ffff603f408 in middle::trans::base::with_scope_result::_14c7edc7eee2f21::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#12 0x00007ffff6080c15 in middle::trans::callee::trans_call_inner::_67b8034bab360ac::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#13 0x00007ffff60b7bcf in middle::trans::expr::trans_overloaded_op::_6519f25aaf593e4::_0$x2e8$x2dpre ()
---Type <return> to continue, or q <return> to quit---
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#14 0x00007ffff60b1077 in middle::trans::expr::trans_assign_op::_7ec41527fd36ec84::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#15 0x00007ffff60a9dce in middle::trans::expr::trans_rvalue_stmt_unadjusted::_104d12f0a1705e95::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#16 0x00007ffff6033048 in middle::trans::expr::trans_into::_96f726277f568b6::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#17 0x00007ffff60316c7 in middle::trans::base::trans_stmt::_1989ce2f7584bf94::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#18 0x00007ffff602f79e in middle::trans::controlflow::trans_block::_f2d871ae4ab77770::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#19 0x00007ffff6142e63 in middle::trans::base::trans_closure::_54779ddb14913af8::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#20 0x00007ffff5ffcc35 in middle::trans::base::trans_fn::_34f4f57a46788272::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#21 0x00007ffff5ff5809 in middle::trans::base::trans_item::_5516fd51ed144c0::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#22 0x00007ffff614944d in middle::trans::base::trans_mod::_19a1ff1b36e4f326::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#23 0x00007ffff6159213 in middle::trans::base::trans_crate::_ad6e822415ada776::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#24 0x00007ffff6790a93 in driver::driver::phase_4_translate_to_llvm::_98876058202a4b9a::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#25 0x00007ffff679189e in driver::driver::compile_input::_71707bfacfa5c237::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#26 0x00007ffff67b28b9 in run_compiler::_3c3c9a54ca89072::_0$x2e8$x2dpre ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#27 0x00007ffff67d472e in main::anon::expr_fn_101450 ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#28 0x00007ffff67d04e4 in monitor::anon::expr_fn_101208 ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#29 0x00007ffff67c76a2 in task::__extensions__::try_100078::anon::expr_fn_100496 ()
   from /home/chris/opt/rust/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#30 0x00007ffff781f3ad in task::spawn::spawn_raw_oldsched::make_child_wrapper::anon::expr_fn_22342 ()
   from /home/chris/opt/rust/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
---Type <return> to continue, or q <return> to quit---
#31 0x00007ffff5c86932 in task_start_wrapper (a=0x7ffff24d2820) at /home/chris/vc/rust/src/rt/rust_task.cpp:162
#32 0x0000000000000000 in ?? ()
