rust
        // Copy libLLVM.so to the target lib dir as well, so the RPATH like
        // `$ORIGIN/../lib` can find it. It may also be used as a dependency
        // of `rustc-dev` to support the inherited `-lLLVM` when using the
        // compiler libraries.
        maybe_install_llvm(builder, target, &image.join("lib"));
