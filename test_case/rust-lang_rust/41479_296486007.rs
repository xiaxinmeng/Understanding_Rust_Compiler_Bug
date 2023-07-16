
INFO:rustc_trans::base: trans_instance(<coproduct::Coproduct<CH, CTail> as coproduct::CoproductFoldable<frunk_core::hlist::HCons<F, FTail>, R>><[closure@src/coproduct.rs:368:38: 368:62], std::string::String, frunk_core::hlist::HCons<[closure@src/coproduct.rs:369:38: 369:64], frunk_core::hlist::HCons<[closure@src/coproduct.rs:370:38: 370:81], frunk_core::hlist::HNil>>, i32, coproduct::Coproduct<f32, coproduct::Coproduct<bool, coproduct::CNil>>>::fold)

#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:51
#1  0x00007ffff735c3fa in __GI_abort () at abort.c:89
#2  0x00007ffff7353e37 in __assert_fail_base (fmt=<optimized out>, 
    assertion=assertion@entry=0x7ffff3406a40 "ReqTy && \"extractvalue indices invalid!\"", 
    file=file@entry=0x7ffff34041b8 "rust://rust/src/llvm/lib/IR/Constants.cpp", line=line@entry=2096, 
    function=function@entry=0x7ffff3407f00 <llvm::ConstantExpr::getExtractValue(llvm::Constant*, llvm::ArrayRef<unsigned int>, llvm::Type*)::__PRETTY_FUNCTION__> "static llvm::Constant* llvm::ConstantExpr::getExtractValue(llvm::Constant*, llvm::ArrayRef<unsigned int>, llvm::Type*)") at assert.c:92
#3  0x00007ffff7353ee2 in __GI___assert_fail (
    assertion=0x7ffff3406a40 "ReqTy && \"extractvalue indices invalid!\"", 
    file=0x7ffff34041b8 "rust://rust/src/llvm/lib/IR/Constants.cpp", 
    line=2096, 
    function=0x7ffff3407f00 <llvm::ConstantExpr::getExtractValue(llvm::Constant*, llvm::ArrayRef<unsigned int>, llvm::Type*)::__PRETTY_FUNCTION__> "static llvm::Constant* llvm::ConstantExpr::getExtractValue(llvm::Constant*, llvm::ArrayRef<unsigned int>, llvm::Type*)") at assert.c:101
#4  0x00007ffff24b4c23 in llvm::ConstantExpr::getExtractValue(llvm::Constant*, llvm::ArrayRef<unsigned int>, llvm::Type*) ()
   from rust://rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-6974c8fa2ebb71c2.so
#5  0x00007ffff24c0279 in LLVMBuildExtractValue ()
   from rust://rust/build-debug-assertions/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-6974c8fa2ebb71c2.so
#6  0x00007ffff6eb1f22 in rustc_trans::builder::Builder::extract_value ()
    at rust://rust/src/librustc_trans/builder.rs:956
#7  rustc_trans::mir::operand::OperandRef::unpack_if_pair ()
    at rust://rust/src/librustc_trans/mir/operand.rs:159
#8  0x00007ffff6eb25c7 in rustc_trans::mir::MirContext::trans_consume ()
    at rust://rust/src/librustc_trans/mir/operand.rs:255
#9  0x00007ffff6eb27c3 in rustc_trans::mir::MirContext::trans_operand ()
    at rust://rust/src/librustc_trans/mir/operand.rs:279
#10 0x00007ffff6eb3dba in rustc_trans::mir::MirContext::trans_rvalue_operand ()
    at rust://rust/src/librustc_trans/mir/rvalue.rs:462
#11 0x00007ffff6e9ff44 in rustc_trans::mir::MirContext::trans_statement ()
    at rust://rust/src/librustc_trans/mir/statement.rs:52
#12 rustc_trans::mir::MirContext::trans_block ()
    at rust://rust/src/librustc_trans/mir/block.rs:109
#13 0x00007ffff6e9c258 in rustc_trans::mir::trans_mir ()
    at rust://rust/src/librustc_trans/mir/mod.rs:326
#14 0x00007ffff6ec12ce in rustc_trans::base::trans_instance ()
    at rust://rust/src/librustc_trans/base.rs:608
#15 rustc_trans::trans_item::TransItem::define ()
    at rust://rust/src/librustc_trans/trans_item.rs:103
#16 0x00007ffff6e4fbd1 in rustc_trans::base::trans_crate::module_translation ()
    at rust://rust/src/librustc_trans/base.rs:1189
#17 rustc::dep_graph::graph::DepGraph::with_task<rustc::dep_graph::safe::AssertDepGraphSafe<&rustc_trans::context::SharedCrateContext>,rustc::dep_graph::safe::AssertDepGraphSafe<rustc_trans::partitioning::CodegenUnit>,(rustc_trans::context::Stats, rustc_trans::ModuleTranslation)> ()
    at rust://rust/src/librustc/dep_graph/graph.rs:111
#18 rustc_trans::base::trans_crate::{{closure}} ()
    at rust://rust/src/librustc_trans/base.rs:1122
