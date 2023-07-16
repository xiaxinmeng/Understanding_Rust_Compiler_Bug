 rust
(gdb) bt
#0  __libc_do_syscall () at ../ports/sysdeps/unix/sysv/linux/arm/libc-do-syscall.S:44
#1  0xb661bf5e in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
#2  0xb661e7b6 in __GI_abort () at abort.c:89
#3  0xb6617190 in __assert_fail_base (fmt=0x1 <error: Cannot access memory at address 0x1>, 
    assertion=0xb4cb3bdc "getOperand(0)->getType() == cast<PointerType>(getOperand(1)->getType())->getElementType() && \"Ptr must be a pointer to Val type!\"", assertion@entry=0x0, 
    file=0xb4cb2f84 "src/llvm/lib/IR/Instructions.cpp", file@entry=0xb04bd2b0 "\001", line=1304, line@entry=3060637860, 
    function=function@entry=0xb4cb712c <llvm::StoreInst::AssertOK()::__PRETTY_FUNCTION__> "void llvm::StoreInst::AssertOK()")
    at assert.c:92
#4  0xb6617226 in __GI___assert_fail (assertion=0x0, file=0xb04bd2b0 "\001", line=3060637860, 
    function=0xb4cb712c <llvm::StoreInst::AssertOK()::__PRETTY_FUNCTION__> "void llvm::StoreInst::AssertOK()") at assert.c:101
#5  0xb3f82f14 in llvm::StoreInst::AssertOK() () from /../lib/librustc_llvm-9026086f.so
#6  0xb3f83150 in llvm::StoreInst::StoreInst(llvm::Value*, llvm::Value*, bool, unsigned int, llvm::AtomicOrdering, llvm::SynchronizationScope, llvm::Instruction*) () from /../lib/librustc_llvm-9026086f.so
#7  0xb3f83198 in llvm::StoreInst::StoreInst(llvm::Value*, llvm::Value*, bool, unsigned int, llvm::Instruction*) ()
   from /../lib/librustc_llvm-9026086f.so
#8  0xb3f831c4 in llvm::StoreInst::StoreInst(llvm::Value*, llvm::Value*, bool, llvm::Instruction*) ()
   from /../lib/librustc_llvm-9026086f.so
#9  0xb3ddfcd4 in LLVMBuildStore () from /../lib/librustc_llvm-9026086f.so
#10 0xb60e9578 in rustc_trans::builder::{{impl}}::store (self=0xb0496e18, val=0x898fac5c, ptr=0x898ce3f4)
    at src/librustc_trans/builder.rs:546
#11 0xb611b9f0 in rustc_trans::build::Store (cx=0x90285930, val=0x898fac5c, ptr=0x898ce3f4) at src/librustc_trans/build.rs:680
#12 0xb61706d4 in rustc_trans::base::store_fat_ptr (cx=0x90285930, data=0x898fac5c, extra=0x8981f81c, dst=0x898fabc4, _ty=0x986948d8)
    at src/librustc_trans/base.rs:978
#13 0xb616fcb0 in rustc_trans::base::coerce_unsized_into (bcx=0x90285930, src=0x898fab1c, src_ty=0x979099dc, dst=0x898fabc4, 
    dst_ty=0x986948d8) at src/librustc_trans/base.rs:626
#14 0xb6170504 in rustc_trans::base::coerce_unsized_into (bcx=0x90285930, src=0x898faa6c, src_ty=0x979099f8, dst=0x898faac4, 
    dst_ty=0x986948f4) at src/librustc_trans/base.rs:659
#15 0xb6170504 in rustc_trans::base::coerce_unsized_into (bcx=0x90285930, src=0x8981f784, src_ty=0x979099c0, dst=0x8981f7dc, 
    dst_ty=0x986947c0) at src/librustc_trans/base.rs:659
#16 0xb6170504 in rustc_trans::base::coerce_unsized_into (bcx=0x90285930, src=0x90294edc, src_ty=0x97909854, dst=0x90287974, 
    dst_ty=0xaa6a213c) at src/librustc_trans/base.rs:659
#17 0xb6450d18 in fnfn (bcx=0x90285930) at src/librustc_trans/mir/rvalue.rs:89
#18 0xb6450bc4 in rustc_trans::common::{{impl}}::with_block<closure,()> (self=0xb0498778, f=...) at src/librustc_trans/common.rs:664
#19 0xb644d11c in rustc_trans::mir::rvalue::{{impl}}::trans_rvalue (self=0xb0499ce4, bcx=..., dest=..., rvalue=0x94199f50)
    at src/librustc_trans/mir/rvalue.rs:71
#20 0xb6441d2c in rustc_trans::mir::statement::{{impl}}::trans_statement (self=0xb0499ce4, bcx=..., statement=0x94199f30)
    at src/librustc_trans/mir/statement.rs:31
#21 0xb643797c in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb0499ce4, bb=...) at src/librustc_trans/mir/block.rs:54
#22 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb049a318) at src/librustc_trans/mir/mod.rs:172
#23 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0xab0a5bb8, body=0xab0acd70, llfndecl=0x90293ae4, 
    param_substs=0xabb90360, def_id=..., inlined_id=99852, fn_ty=..., abi=Rust, closure_env=...) at src/librustc_trans/base.rs:1837
#24 0xb61b0444 in rustc_trans::base::trans_fn (ccx=0xb04b4e9c, decl=0xab0a5bb8, body=0xab0acd70, llfndecl=0x90293ae4, 
    param_substs=0xabb90360, id=99852) at src/librustc_trans/base.rs:1921
#25 0xb626bd3c in rustc_trans::monomorphize::monomorphic_fn (ccx=0xb04b4e9c, fn_id=..., psubsts=0xabb90360)
    at src/librustc_trans/monomorphize.rs:138
#26 0xb625df64 in rustc_trans::callee::get_fn (ccx=0xb04b4e9c, def_id=..., substs=0xabb90360) at src/librustc_trans/callee.rs:478
#27 0xb625c9a8 in rustc_trans::callee::{{impl}}::trait_method (ccx=0xb04b4e9c, def_id=..., substs=0x96463898)
    at src/librustc_trans/callee.rs:173
#28 0xb616c8b4 in rustc_trans::callee::{{impl}}::def (ccx=0xb04b4e9c, def_id=..., substs=0x96463898)
    at src/librustc_trans/callee.rs:118
#29 0xb6438fcc in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb049d6fc, bb=...) at src/librustc_trans/mir/block.rs:161
#30 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb049dd30) at src/librustc_trans/mir/mod.rs:172
#31 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0x902ca018, body=0x902e0410, llfndecl=0x90280f84, 
    param_substs=0x96463848, def_id=..., inlined_id=4619003, fn_ty=..., abi=Rust, closure_env=...) at src/librustc_trans/base.rs:1837
#32 0xb61b0444 in rustc_trans::base::trans_fn (ccx=0xb04b4e9c, decl=0x902ca018, body=0x902e0410, llfndecl=0x90280f84, 
    param_substs=0x96463848, id=4619003) at src/librustc_trans/base.rs:1921
#33 0xb626bd3c in rustc_trans::monomorphize::monomorphic_fn (ccx=0xb04b4e9c, fn_id=..., psubsts=0x96463848)
    at src/librustc_trans/monomorphize.rs:138
#34 0xb625df64 in rustc_trans::callee::get_fn (ccx=0xb04b4e9c, def_id=..., substs=0x96463848) at src/librustc_trans/callee.rs:478
#35 0xb616ca4c in rustc_trans::callee::{{impl}}::def (ccx=0xb04b4e9c, def_id=..., substs=0x96463848)
    at src/librustc_trans/callee.rs:140
#36 0xb6438fcc in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb04a0d0c, bb=...) at src/librustc_trans/mir/block.rs:161
#37 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb04a1340) at src/librustc_trans/mir/mod.rs:172
#38 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0x89fedff0, body=0x9025b440, llfndecl=0x9025b9e4, 
    param_substs=0x96463848, def_id=..., inlined_id=4617005, fn_ty=..., abi=Rust, closure_env=...) at src/librustc_trans/base.rs:1837
#39 0xb61b0444 in rustc_trans::base::trans_fn (ccx=0xb04b4e9c, decl=0x89fedff0, body=0x9025b440, llfndecl=0x9025b9e4, 
    param_substs=0x96463848, id=4617005) at src/librustc_trans/base.rs:1921
#40 0xb626bd3c in rustc_trans::monomorphize::monomorphic_fn (ccx=0xb04b4e9c, fn_id=..., psubsts=0x96463848)
    at src/librustc_trans/monomorphize.rs:138
#41 0xb625df64 in rustc_trans::callee::get_fn (ccx=0xb04b4e9c, def_id=..., substs=0x96463848) at src/librustc_trans/callee.rs:478
#42 0xb616ca4c in rustc_trans::callee::{{impl}}::def (ccx=0xb04b4e9c, def_id=..., substs=0x96463848)
    at src/librustc_trans/callee.rs:140
#43 0xb6438fcc in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb04a431c, bb=...) at src/librustc_trans/mir/block.rs:161
#44 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb04a4950) at src/librustc_trans/mir/mod.rs:172
#45 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0x90254c28, body=0x90240db0, llfndecl=0x90240fac, 
    param_substs=0x96463848, def_id=..., inlined_id=4616960, fn_ty=..., abi=Rust, closure_env=...) at src/librustc_trans/base.rs:1837
#46 0xb61b0444 in rustc_trans::base::trans_fn (ccx=0xb04b4e9c, decl=0x90254c28, body=0x90240db0, llfndecl=0x90240fac, 
    param_substs=0x96463848, id=4616960) at src/librustc_trans/base.rs:1921
#47 0xb626bd3c in rustc_trans::monomorphize::monomorphic_fn (ccx=0xb04b4e9c, fn_id=..., psubsts=0x96463848)
    at src/librustc_trans/monomorphize.rs:138
#48 0xb625df64 in rustc_trans::callee::get_fn (ccx=0xb04b4e9c, def_id=..., substs=0x96463848) at src/librustc_trans/callee.rs:478
#49 0xb616ca4c in rustc_trans::callee::{{impl}}::def (ccx=0xb04b4e9c, def_id=..., substs=0x96463848)
    at src/librustc_trans/callee.rs:140
#50 0xb6438fcc in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb04a792c, bb=...) at src/librustc_trans/mir/block.rs:161
#51 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb04a7f60) at src/librustc_trans/mir/mod.rs:172
#52 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0xab09cb78, body=0xab09f608, llfndecl=0x89f1f2dc, 
    param_substs=0xabb90360, def_id=..., inlined_id=99459, fn_ty=..., abi=Rust, closure_env=...) at src/librustc_trans/base.rs:1837
#53 0xb61b0444 in rustc_trans::base::trans_fn (ccx=0xb04b4e9c, decl=0xab09cb78, body=0xab09f608, llfndecl=0x89f1f2dc, 
    param_substs=0xabb90360, id=99459) at src/librustc_trans/base.rs:1921
#54 0xb626bd3c in rustc_trans::monomorphize::monomorphic_fn (ccx=0xb04b4e9c, fn_id=..., psubsts=0xabb90360)
    at src/librustc_trans/monomorphize.rs:138
#55 0xb625df64 in rustc_trans::callee::get_fn (ccx=0xb04b4e9c, def_id=..., substs=0xabb90360) at src/librustc_trans/callee.rs:478
#56 0xb616ca4c in rustc_trans::callee::{{impl}}::def (ccx=0xb04b4e9c, def_id=..., substs=0xabb90360)
    at src/librustc_trans/callee.rs:140
#57 0xb6438fcc in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb04aaf3c, bb=...) at src/librustc_trans/mir/block.rs:161
#58 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb04ab570) at src/librustc_trans/mir/mod.rs:172
#59 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0xb04ac31c, body=0xb04ac2fc, llfndecl=0x89f1167c, 
    param_substs=0xabb90360, def_id=..., inlined_id=4294967295, fn_ty=..., abi=RustCall, closure_env=...)
    at src/librustc_trans/base.rs:1837
#60 0xb62a080c in rustc_trans::closure::trans_closure_expr (dest=..., decl=0xb04ac31c, body=0xb04ac2fc, id=4294967295, 
    closure_def_id=..., closure_substs=0xb04ac2e4) at src/librustc_trans/closure.rs:239
#61 0xb644d900 in rustc_trans::mir::rvalue::{{impl}}::trans_rvalue (self=0xb04adbdc, bcx=..., dest=..., rvalue=0x941c0a48)
    at src/librustc_trans/mir/rvalue.rs:143
#62 0xb6441d2c in rustc_trans::mir::statement::{{impl}}::trans_statement (self=0xb04adbdc, bcx=..., statement=0x941c0a28)
    at src/librustc_trans/mir/statement.rs:31
#63 0xb643797c in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb04adbdc, bb=...) at src/librustc_trans/mir/block.rs:54
#64 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb04ae210) at src/librustc_trans/mir/mod.rs:172
#65 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0xb04aefbc, body=0xb04aef9c, llfndecl=0x8a1b4874, 
    param_substs=0xabb90360, def_id=..., inlined_id=4294967295, fn_ty=..., abi=RustCall, closure_env=...)
    at src/librustc_trans/base.rs:1837
#66 0xb62a080c in rustc_trans::closure::trans_closure_expr (dest=..., decl=0xb04aefbc, body=0xb04aef9c, id=4294967295, 
    closure_def_id=..., closure_substs=0xb04aef84) at src/librustc_trans/closure.rs:239
#67 0xb644d900 in rustc_trans::mir::rvalue::{{impl}}::trans_rvalue (self=0xb04b087c, bcx=..., dest=..., rvalue=0x941c0f88)
    at src/librustc_trans/mir/rvalue.rs:143
#68 0xb6441d2c in rustc_trans::mir::statement::{{impl}}::trans_statement (self=0xb04b087c, bcx=..., statement=0x941c0f68)
    at src/librustc_trans/mir/statement.rs:31
#69 0xb643797c in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb04b087c, bb=...) at src/librustc_trans/mir/block.rs:54
#70 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb04b0eb0) at src/librustc_trans/mir/mod.rs:172
#71 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0xab0b39e8, body=0xab0b5010, llfndecl=0x8a135914, 
    param_substs=0xabb90360, def_id=..., inlined_id=100311, fn_ty=..., abi=Rust, closure_env=...) at src/librustc_trans/base.rs:1837
#72 0xb61b0444 in rustc_trans::base::trans_fn (ccx=0xb04b4e9c, decl=0xab0b39e8, body=0xab0b5010, llfndecl=0x8a135914, 
    param_substs=0xabb90360, id=100311) at src/librustc_trans/base.rs:1921
#73 0xb626bd3c in rustc_trans::monomorphize::monomorphic_fn (ccx=0xb04b4e9c, fn_id=..., psubsts=0xabb90360)
    at src/librustc_trans/monomorphize.rs:138
#74 0xb625df64 in rustc_trans::callee::get_fn (ccx=0xb04b4e9c, def_id=..., substs=0xabb90360) at src/librustc_trans/callee.rs:478
#75 0xb616ca4c in rustc_trans::callee::{{impl}}::def (ccx=0xb04b4e9c, def_id=..., substs=0xabb90360)
    at src/librustc_trans/callee.rs:140
#76 0xb6438fcc in rustc_trans::mir::block::{{impl}}::trans_block (self=0xb04b3e8c, bb=...) at src/librustc_trans/mir/block.rs:161
#77 0xb61aea7c in rustc_trans::mir::trans_mir (fcx=0xb04b44c0) at src/librustc_trans/mir/mod.rs:172
#78 0xb61ab9f8 in rustc_trans::base::trans_closure (ccx=0xb04b4e9c, decl=0xab0d0a38, body=0xab0d10a0, llfndecl=0x8a1a118c, 
    param_substs=0xabd1cb08, def_id=..., inlined_id=101483, fn_ty=..., abi=Rust, closure_env=...) at src/librustc_trans/base.rs:1837
#79 0xb61b0444 in rustc_trans::base::trans_fn (ccx=0xb04b4e9c, decl=0xab0d0a38, body=0xab0d10a0, llfndecl=0x8a1a118c, 
    param_substs=0xabd1cb08, id=101483) at src/librustc_trans/base.rs:1921
#80 0xb61cde18 in rustc_trans::base::trans_item (ccx=0xb04b53e4, item=0xab0d04bc) at src/librustc_trans/base.rs:2268
#81 0xb62236d0 in fnfn () at src/librustc_trans/base.rs:2928
#82 0xb62235ec in rustc_trans::dep_graph::{{impl}}::with_task<closure,()> (self=0xb04b7c28, key=..., op=...)
    at src/librustc/dep_graph/mod.rs:165
#83 0xb6223534 in rustc_trans::base::{{impl}}::visit_item (self=0xb04b53dc, i=0xab0d04bc) at src/librustc_trans/base.rs:2918
#84 0xb620daf0 in rustc_trans::base::{{impl}}::visit_nested_item (self=0xb04b53dc, item_id=...) at src/librustc_trans/base.rs:2904
#85 0xb620d66c in rustc_trans::hir::intravisit::walk_mod<rustc_trans::base::TransItemsWithinModVisitor> (visitor=0xb04b53dc, 
    module=0xab0cca38) at src/librustc/hir/intravisit.rs:239
#86 0xb62219b0 in rustc_trans::hir::intravisit::Visitor::visit_mod<rustc_trans::base::TransItemsWithinModVisitor> (self=0xb04b53dc, 
    m=0xab0cca38, _s=..., _n=101332) at src/librustc/hir/intravisit.rs:104
#87 0xb621be8c in rustc_trans::hir::intravisit::walk_item<rustc_trans::base::TransItemsWithinModVisitor> (visitor=0xb04b53dc, 
    item=0xab0cca24) at src/librustc/hir/intravisit.rs:335
#88 0xb620dc20 in rustc_trans::base::{{impl}}::visit_item (self=0xb04b5ce8, i=0xab0cca24) at src/librustc_trans/base.rs:2889
#89 0xb620db88 in rustc_trans::hir::{{impl}}::visit_all_items<rustc_trans::base::TransModVisitor> (self=0xb04b9c8c, visitor=0xb04b5ce8)
    at src/librustc/hir/mod.rs:480
#90 0xb61e75ac in rustc_trans::base::trans_crate (tcx=0xb04b7bd4, mir_map=0xb04b63e4, analysis=...) at src/librustc_trans/base.rs:2760
#91 0xb6e82d74 in fnfn () at src/librustc_driver/driver.rs:957
#92 0xb6e82748 in rustc_driver::util::common::time<rustc_trans::CrateTranslation,closure> (do_it=false, what=..., f=...)
    at src/librustc/util/common.rs:38
#93 0xb6cb6b9c in rustc_driver::driver::phase_4_translate_to_llvm (tcx=0xb04b7bd4, mir_map=..., analysis=...)
    at src/librustc_driver/driver.rs:955
#94 0xb6ca9670 in fnfn (tcx=0xb04b7bd4, mir_map=..., analysis=..., result=...) at src/librustc_driver/driver.rs:206
#95 0xb6c7e1f0 in fnfn (tcx=0xb04b7bd4) at src/librustc_driver/driver.rs:930
#96 0xb6c7d3b4 in rustc::ty::context::tls::enter::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hdb0fc61b0612e833 ()
    at src/librustc_driver/driver.rs:816
#97 0xb6c7d230 in rustc_driver::thread::local::{{impl}}::with<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (
    self=0xb1f8ad54 <rustc::ty::context::tls::TLS_TCX::h75f0323020f314c8>, f=...) at src/libstd/thread/local.rs:211
#98 0xb6c7cd74 in rustc::ty::context::tls::enter::_$u7b$$u7b$closure$u7d$$u7d$::h1cd1ae180cde5f0d ()
    at src/librustc_driver/driver.rs:816
#99 0xb6c7cbf0 in rustc_driver::thread::local::{{impl}}::with<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (self=0xb0f68d58 <syntax::codemap::SPAN_DEBUG::h6236c52aef672e24>, 
    f=...) at src/libstd/thread/local.rs:211
#100 0xb6c7ca20 in rustc_driver::ty::context::tls::enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (tcx=..., f=...) at src/librustc/ty/context.rs:660
#101 0xb6c7996c in rustc_driver::ty::context::{{impl}}::create_and_enter<closure,core::result::Result<core::result::Result<(rustc::sessi
on::config::OutputFilenames, rustc_trans::CrateTranslation), usize>, usize>> (s=0xb04baf10, arenas=0xb04b9bb8, def_map=..., 
    named_region_map=..., map=..., freevars=..., region_maps=..., lang_items=..., stability=..., crate_name=..., f=...)
    at src/librustc/ty/context.rs:538
#102 0xb6c6b018 in rustc_driver::driver::phase_3_run_analysis_passes<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::CrateTranslation), usize>> (sess=0xb04baf10, cstore=0xafb16700, hir_map=..., arenas=0xb04b9bb8, name=..., 
    make_glob_map=No, f=...) at src/librustc_driver/driver.rs:820
#103 0xb6c00328 in rustc_driver::driver::compile_input (sess=0xb04baf10, cstore=0xafb16700, cfg=..., input=0xb04bb6a8, 
    outdir=0xb04bb794, output=0xb04bb788, addl_plugins=..., control=0xb04ba4ec) at src/librustc_driver/driver.rs:172
#104 0xb6be3e98 in rustc_driver::run_compiler (args=..., callbacks=...) at src/librustc_driver/lib.rs:213
#105 0xb6be07b8 in fnfn () at src/librustc_driver/lib.rs:132
#106 0xb6bde8d0 in fnfn () at src/librustc_driver/lib.rs:997
#107 0xb6bde3a8 in std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h71e29f932f68a9c9 ()
    at src/libstd/sys/common/mutex.rs:28
#108 0xb6bde340 in rustc_driver::sys_common::unwind::try::try_fn<closure> (opt_closure=0xb04bcb08 "")
    at src/libstd/sys/common/unwind/mod.rs:127
#109 0xb6857d6c in __rust_try () from /../lib/libstd-9026086f.so
#110 0xb6857a54 in fnfn (s=0xb04bd7a8) at src/libstd/sys/common/unwind/mod.rs:148
#111 0xb6857950 in std::thread::local::{{impl}}::with<closure,core::result::Result<(), Box<Any>>> (
    self=0xb69bcab0 <std::panicking::PANIC_COUNT::hb97300d99929c675>, f=...) at src/libstd/thread/local.rs:211
#112 0xb6857858 in std::sys_common::unwind::inner_try (f=0xb6bde308 <rustc_driver::sys_common::unwind::try::try_fn<closure>>, 
    data=0xb04bcb08 "") at src/libstd/sys/common/unwind/mod.rs:133
#113 0xb6bde2a8 in rustc_driver::sys_common::unwind::try<closure> (f=...) at src/libstd/sys/common/unwind/mod.rs:123
#114 0xb6bde154 in std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::h98fc4c71843fb136 () at src/libstd/sys/common/mutex.rs:28
#115 0xb6bdeba8 in rustc_driver::boxed::{{impl}}::call_box (self=0x7f5725a0, args=0) at src/liballoc/boxed.rs:541
#116 0xb6846a38 in std::boxed::{{impl}}::call_once (self=..., args=0) at src/liballoc/boxed.rs:550
#117 0xb6854fb8 in std::sys_common::thread::start_thread (main=0x7f572660) at src/libstd/sys/common/thread.rs:23
#118 0xb6892798 in std::sys::thread::{{impl}}::new::thread_start (main=0x7f572660) at src/libstd/sys/unix/thread.rs:74
#119 0xb07dafbc in start_thread (arg=0xb04bd2b0) at pthread_create.c:314
#120 0xb668951c in ?? () at ../ports/sysdeps/unix/sysv/linux/arm/nptl/../clone.S:92 from /lib/arm-linux-gnueabihf/libc.so.6
