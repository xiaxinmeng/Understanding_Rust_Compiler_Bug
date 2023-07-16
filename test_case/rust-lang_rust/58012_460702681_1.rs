
#9  0x00007f4aaca9de3a in llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level) ()
   from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007f4aacc13c00 in llvm::SelectionDAGISel::CodeGenAndEmitDAG() ()
   from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007f4aacc195b5 in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) ()
   from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007f4aacc1b2c2 in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) [clone .part.987] ()
   from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007f4aace64d2e in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) [clone .part.41] ()
   from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007f4aad96af09 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007f4aad96afb9 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007f4aad96a338 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007f4aab9f5b04 in LLVMRustWriteOutputFile () from /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#18 0x00007f4aab90fa8c in rustc_codegen_llvm::back::write::write_output_file (handler=0x7f4aa83a7e20, target=0x7f4a580a4030, pm=0x7f4a5816dc40, m=0x0, 
    output=<error reading variable: access outside bounds of object referenced via synthetic pointer>, file_type=rustc_codegen_llvm::llvm_::ffi::FileType::ObjectFile)
    at src/librustc_codegen_llvm/back/write.rs:76
#19 0x00007f4aab9435cc in rustc_codegen_llvm::back::write::codegen::{{closure}}::{{closure}} (cpm=0x7f4a5803c580) at src/librustc_codegen_llvm/back/write.rs:592
#20 rustc_codegen_llvm::back::write::codegen::with_codegen (tm=<optimized out>, llmod=<optimized out>, f=..., no_builtins=<optimized out>)
    at src/librustc_codegen_llvm/back/write.rs:479
#21 rustc_codegen_llvm::back::write::codegen::{{closure}} () at src/librustc_codegen_llvm/back/write.rs:591
#22 0x00007f4aab941d0c in rustc::util::common::time_ext (do_it=<optimized out>, 
    sess=<unknown type in /r/r/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so, CU 0x1bc4150, DIE 0x1bd9e25>, 
    what=..., f=...) at /r/r/src/librustc/util/common.rs:150
#23 0x00007f4aab912c09 in rustc_codegen_llvm::back::write::codegen (cgcx=<optimized out>, diag_handler=<optimized out>, module=..., config=<optimized out>, timeline=<optimized out>)
    at src/librustc_codegen_llvm/back/write.rs:528
#24 0x00007f4aab87c15b in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::write::WriteBackendMethods>::codegen (cgcx=<optimized out>, diag_handler=0x0, 
    config=<optimized out>, timeline=<optimized out>, module=...) at src/librustc_codegen_llvm/lib.rs:206
#25 rustc_codegen_ssa::back::write::execute_optimize_work_item (cgcx=<optimized out>, module_config=0x7f4a714db260, timeline=<optimized out>, module=...)
    at /r/r/src/librustc_codegen_ssa/back/write.rs:778
#26 rustc_codegen_ssa::back::write::execute_work_item (cgcx=<optimized out>, work_item=<optimized out>, timeline=0x7f4aa83a8080) at /r/r/src/librustc_codegen_ssa/back/write.rs:703
#27 0x00007f4aab86bb78 in rustc_codegen_ssa::back::write::spawn_work::{{closure}} () at /r/r/src/librustc_codegen_ssa/back/write.rs:1570
