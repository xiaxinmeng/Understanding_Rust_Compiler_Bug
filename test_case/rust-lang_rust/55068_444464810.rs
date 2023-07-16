
#0  0x00007fffee33d626 in llvm::ValueHandleBase::AddToExistingUseList(llvm::ValueHandleBase**) ()
   from /home/nikic/.rustup/toolchains/1ecf6929dc3b309cdfcb7239260777dab38242b9-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007fffecf8bc99 in llvm::scc_iterator<llvm::CallGraph*, llvm::GraphTraits<llvm::CallGraph*> >::DFSVisitChildren() ()
   from /home/nikic/.rustup/toolchains/1ecf6929dc3b309cdfcb7239260777dab38242b9-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007fffecf8b0e8 in llvm::scc_iterator<llvm::CallGraph*, llvm::GraphTraits<llvm::CallGraph*> >::GetNextSCC() ()
   from /home/nikic/.rustup/toolchains/1ecf6929dc3b309cdfcb7239260777dab38242b9-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fffedf31046 in (anonymous namespace)::CGPassManager::runOnModule(llvm::Module&) ()
   from /home/nikic/.rustup/toolchains/1ecf6929dc3b309cdfcb7239260777dab38242b9-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fffee2f1df0 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/nikic/.rustup/toolchains/1ecf6929dc3b309cdfcb7239260777dab38242b9-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fffee24f17b in LLVMRunPassManager ()
   from /home/nikic/.rustup/toolchains/1ecf6929dc3b309cdfcb7239260777dab38242b9-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fffec5a7da6 in rustc_codegen_llvm::back::write::execute_work_item ()
   from /home/nikic/.rustup/toolchains/1ecf6929dc3b309cdfcb7239260777dab38242b9-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
