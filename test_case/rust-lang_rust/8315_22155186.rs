 text
#0  0x00007ffff59df037 in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
#1  0x00007ffff59e2698 in __GI_abort () at abort.c:90
#2  0x00007ffff59d7e03 in __assert_fail_base (fmt=0x7ffff5b2e578 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n", 
    assertion=assertion@entry=0x7ffff4a18920 "(i >= FTy->getNumParams() || FTy->getParamType(i) == Args[i]->getType()) && \"Calling a function with a bad signature!\"", 
    file=file@entry=0x7ffff4a16c48 "/build/buildd/rust-nightly-201308040649~93432a2~raring/src/llvm/lib/IR/Instructions.cpp", line=line@entry=281, 
    function=function@entry=0x7ffff4a19a80 "void llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&)") at assert.c:92
#3  0x00007ffff59d7eb2 in __GI___assert_fail (
    assertion=0x7ffff4a18920 "(i >= FTy->getNumParams() || FTy->getParamType(i) == Args[i]->getType()) && \"Calling a function with a bad signature!\"", 
    file=0x7ffff4a16c48 "/build/buildd/rust-nightly-201308040649~93432a2~raring/src/llvm/lib/IR/Instructions.cpp", line=281, 
    function=0x7ffff4a19a80 "void llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&)") at assert.c:101
#4  0x00007ffff435d60e in llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&)
    () from /usr/lib/rust/rust-nightly/bin/../lib/./librustllvm.so
#5  0x00007ffff3f7a199 in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/./librustllvm.so
#6  0x00007ffff42dfe56 in LLVMBuildCall () from /usr/lib/rust/rust-nightly/bin/../lib/./librustllvm.so
#7  0x00007ffff61344ca in middle::trans::build::Call::_fdfd495e79d29b8::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#8  0x00007ffff617479a in middle::trans::base::invoke::_e6f3e84a964ff35::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#9  0x00007ffff6172b3d in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#10 0x00007ffff6122682 in middle::trans::base::with_scope_result::_cf2bf7b5bddd954::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#11 0x00007ffff615d5e5 in middle::trans::callee::trans_call_inner::_2014fbb37953::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#12 0x00007ffff612c9fe in middle::trans::callee::trans_lang_call::_b953316cd5a1d0f6::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#13 0x00007ffff623db9d in middle::trans::_match::compare_values::_5c865a44a26bc8b::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#14 0x00007ffff6245bf8 in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#15 0x00007ffff6122682 in middle::trans::base::with_scope_result::_cf2bf7b5bddd954::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#16 0x00007ffff6242e46 in middle::trans::_match::compile_submatch_continue::_f75a2bdd7c45449::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#17 0x00007ffff623f8b1 in middle::trans::_match::compile_submatch::_faa543787be36391::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#18 0x00007ffff624708c in middle::trans::_match::trans_match_inner::_c9d4d3c3e8dbfd9d::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#19 0x00007ffff624644e in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#20 0x00007ffff611bccb in middle::trans::base::with_scope::_676917add84a1f3::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#21 0x00007ffff618bdde in middle::trans::_match::trans_match::_9b537166e48aa7b0::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#22 0x00007ffff61812bd in middle::trans::expr::trans_rvalue_dps_unadjusted::_ab23c5381915e526::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#23 0x00007ffff6177f56 in middle::trans::expr::trans_to_datum_unadjusted::_9aee8781b2414713::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#24 0x00007ffff611a581 in middle::trans::expr::trans_to_datum::_9aee8781b2414713::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#25 0x00007ffff6175290 in middle::trans::callee::trans_arg_expr::_52c79f83f7c9ee30::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#26 0x00007ffff617423c in middle::trans::callee::trans_args::_19206a2bff4aae2::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#27 0x00007ffff6172a49 in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#28 0x00007ffff6122682 in middle::trans::base::with_scope_result::_cf2bf7b5bddd954::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#29 0x00007ffff615d5e5 in middle::trans::callee::trans_call_inner::_2014fbb37953::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#30 0x00007ffff615d2ce in middle::trans::callee::trans_call::_1e2a7cadd68d7ddf::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#31 0x00007ffff618082f in middle::trans::expr::trans_rvalue_dps_unadjusted::_ab23c5381915e526::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#32 0x00007ffff6118bdd in middle::trans::expr::trans_into::_ab23c5381915e526::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#33 0x00007ffff6117cbd in middle::trans::base::trans_stmt::_3a9495c55ad3c3ed::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#34 0x00007ffff611629d in middle::trans::controlflow::trans_block::_5cf44455c38459e::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#35 0x00007ffff62159b9 in middle::trans::base::trans_closure::_2699e8c8df8ed03f::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#36 0x00007ffff60e62dd in middle::trans::base::trans_fn::_52391cac56ceafa3::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#37 0x00007ffff60e1639 in middle::trans::base::trans_item::_de39ea11a626c0da::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#38 0x00007ffff621ab4b in middle::trans::base::trans_mod::_36339051249617fd::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#39 0x00007ffff6229cb0 in middle::trans::base::trans_crate::_903da5e2989735e0::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#40 0x00007ffff67c0d13 in driver::driver::phase_4_translate_to_llvm::_fb66772aa77097fe::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#41 0x00007ffff67c16b8 in driver::driver::compile_input::_ffbe5659b7f847f9::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#42 0x00007ffff67e5139 in run_compiler::_8eff87aa729b72::_0$x2e8$x2dpre ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#43 0x00007ffff6803dde in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#44 0x00007ffff67ffc74 in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#45 0x00007ffff67f8e22 in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#46 0x00007ffff782a78d in ?? () from /usr/lib/rust/rust-nightly/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#47 0x00007ffff5d89ffb in task_start_wrapper(spawn_args*) ()
   from /usr/lib/rust/rust-nightly/bin/../lib/librustrt.so
#48 0x0000000000000000 in ?? ()
