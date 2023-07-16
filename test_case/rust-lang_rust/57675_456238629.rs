
cargo:warning=../rustllvm/RustWrapper.cpp: In function ‘LLVMOpaqueMetadata* LLVMRustDIBuilderCreateFunction(LLVMRustDIBuilderRef, LLVMMetadataRef, const char*, const char*, LLVMMetadataRef, unsigned int, LLVMMetadataRef, bool, bool, unsigned int, LLVMRustDIFlags, bool, LLVMValueRef, LLVMMetadataRef, LLVMMetadataRef)’:
cargo:warning=../rustllvm/RustWrapper.cpp:587:38: error: no matching function for call to ‘llvm::DIBuilder::createFunction(llvm::DIScope*, const char*&, const char*&, llvm::DIFile*, unsigned int&, llvm::DISubroutineType*, bool&, bool&, unsigned int&, llvm::DINode::DIFlags, bool&, llvm::DITemplateParameterArray&, llvm::DISubprogram*)’
cargo:warning=       unwrapDIPtr<DISubprogram>(Decl));
cargo:warning=                                      ^
cargo:warning=In file included from ../rustllvm/rustllvm.h:53,
cargo:warning=                 from ../rustllvm/RustWrapper.cpp:1:
cargo:warning=/local_scratch/glaubitz/M680x0-llvm/include/llvm/IR/DIBuilder.h:663:5: note: candidate: ‘llvm::DISubprogram* llvm::DIBuilder::createFunction(llvm::DIScope*, llvm::StringRef, llvm::StringRef, llvm::DIFile*, unsigned int, llvm::DISubroutineType*, unsigned int, llvm::DINode::DIFlags, llvm::DISubprogram::DISPFlags, llvm::DITemplateParameterArray, llvm::DISubprogram*, llvm::DITypeArray)’
cargo:warning=     createFunction(DIScope *Scope, StringRef Name, StringRef LinkageName,
cargo:warning=     ^~~~~~~~~~~~~~
cargo:warning=/local_scratch/glaubitz/M680x0-llvm/include/llvm/IR/DIBuilder.h:663:5: note:   candidate expects 12 arguments, 13 provided
cargo:warning=../rustllvm/RustWrapper.cpp: In function ‘void LLVMRustUnpackOptimizationDiagnostic(LLVMDiagnosticInfoRef, RustStringRef, LLVMOpaqueValue**, unsigned int*, unsigned int*, RustStringRef, RustStringRef)’:
cargo:warning=../rustllvm/RustWrapper.cpp:923:23: error: ‘class llvm::DiagnosticLocation’ has no member named ‘getFilename’; did you mean ‘getLine’?
cargo:warning=     FilenameOS << loc.getFilename();
cargo:warning=                       ^~~~~~~~~~~
cargo:warning=                       getLine
