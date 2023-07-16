rust
        // Get the sysroot, looking from most specific to this invocation to the least:
        // - command line
        // - runtime environment
        //    - SYSROOT
        //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN
        // - sysroot from rustc in the path
        // - compile-time environment
        //    - SYSROOT
        //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN
