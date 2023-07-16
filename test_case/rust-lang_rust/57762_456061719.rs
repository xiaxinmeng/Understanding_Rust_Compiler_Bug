
[Switching to Thread 0x7fffeb9bd700 (LWP 1464)]
0x00007ffff2cf735c in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
(gdb) bt
#0  0x00007ffff2cf735c in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#1  0x00007ffff2cf72a5 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#2  0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#3  0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#4  0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#5  0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#6  0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#7  0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#8  0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#9  0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#10 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#11 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#12 0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#13 0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#14 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#15 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#16 0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#17 0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#18 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#19 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#20 0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#21 0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#22 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#23 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#24 0x00007ffff2cf9ac3 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#25 0x00007ffff2cf75dd in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#26 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#27 0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#28 0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#29 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#30 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#31 0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#32 0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#33 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#34 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#35 0x00007ffff2cf9ac3 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#36 0x00007ffff2cf75dd in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#37 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#38 0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#39 0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#40 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#41 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#42 0x00007ffff2cf9ac3 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#43 0x00007ffff2cf75dd in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#44 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#45 0x00007ffff2cf9284 in llvm::DwarfUnit::constructMemberDIE(llvm::DIE&, llvm::DIDerivedType const*) () from /usr/lib/libLLVM-7.so
#46 0x00007ffff2cf7105 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#47 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#48 0x00007ffff2cf6f56 in llvm::DwarfUnit::constructTypeDIE(llvm::DIE&, llvm::DICompositeType const*) () from /usr/lib/libLLVM-7.so
#49 0x00007ffff2cf75f1 in llvm::DwarfUnit::getOrCreateTypeDIE(llvm::MDNode const*) () from /usr/lib/libLLVM-7.so
#50 0x00007ffff2cf7615 in llvm::DwarfUnit::addType(llvm::DIE&, llvm::DIType const*, llvm::dwarf::Attribute) () from /usr/lib/libLLVM-7.so
#51 0x00007ffff2cd350a in llvm::DwarfCompileUnit::getOrCreateGlobalVariableDIE(llvm::DIGlobalVariable const*, llvm::ArrayRef<llvm::DwarfCompileUnit::GlobalExpr>) () from /usr/lib/libLLVM-7.so
#52 0x00007ffff2cebd13 in llvm::DwarfDebug::beginModule() () from /usr/lib/libLLVM-7.so
#53 0x00007ffff2cb01f9 in llvm::AsmPrinter::doInitialization(llvm::Module&) () from /usr/lib/libLLVM-7.so
#54 0x00007ffff26aec46 in llvm::FPPassManager::doInitialization(llvm::Module&) () from /usr/lib/libLLVM-7.so
#55 0x00007ffff26b9a06 in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /usr/lib/libLLVM-7.so
#56 0x00007ffff5e6aa9a in LLVMRustWriteOutputFile () from /usr/lib64/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#57 0x00007ffff5e0369c in ?? () from /usr/lib64/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#58 0x00007ffff5e05960 in ?? () from /usr/lib64/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#59 0x00007ffff5e5b1f1 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::write::WriteBackendMethods>::codegen ()
   from /usr/lib64/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#60 0x00007ffff5d8a98c in ?? () from /usr/lib64/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#61 0x00007ffff5d8c459 in ?? () from /usr/lib64/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#62 0x00007ffff7db07ba in __rust_maybe_catch_panic () from /usr/lib/libstd-3572d7be0ebc7029.so
#63 0x00007ffff5db5b18 in ?? () from /usr/lib64/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#64 0x00007ffff7da69ae in ?? () from /usr/lib/libstd-3572d7be0ebc7029.so
#65 0x00007ffff6113a9d in start_thread () from /usr/lib/libpthread.so.0
#66 0x00007ffff7c65b23 in clone () from /usr/lib/libc.so.6
