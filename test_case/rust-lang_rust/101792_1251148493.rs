rust
                    if we_are_cross_compiling() {
                        cmd.arg(format!("--target={}", sess.target.llvm_target));
                    }
