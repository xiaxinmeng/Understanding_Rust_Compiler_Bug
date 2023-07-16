rust
        if !tcx.crate_name(LOCAL_CRATE).as_str().starts_with("rustc_") {
            // Only run this pass on the compiler.
            return;
        }
