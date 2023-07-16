rust
if builder.config.skip_llvm_rebuild {
            builder.info("Skipping LLVM rebuild");
            return build_llvm_config
        }
