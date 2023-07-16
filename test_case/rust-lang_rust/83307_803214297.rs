rust
    unsafe {
        llvm::LLVMRustSetLinkage(llfn, llvm::Linkage::ExternalLinkage);
        llvm::LLVMRustSetVisibility(llfn, llvm::Visibility::Hidden);
    }
