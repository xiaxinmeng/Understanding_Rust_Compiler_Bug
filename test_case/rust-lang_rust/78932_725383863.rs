rust
    if !builder.config.llvm_link_shared {
        // We do not need to copy LLVM files into the sysroot if it is not
        // dynamically linked; it is already included into librustc_llvm
        // statically.
        return;
    }
