rust
        /* run only if llvm-config isn't used */
        if let Some(config) = builder.config.target_config.get(&target) {
            if let Some(ref _s) = config.llvm_config {
                builder.info(&format!("Skipping LlvmTools ({}): external LLVM", target));
                return None;
            }
        }
