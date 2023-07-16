plain
    Checking hir-def v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-def)
error: unused variable: `var`
  --> crates/proc-macro-srv/src/abis/abi_sysroot/ra_server.rs:75:36
   |
75 |     fn injected_env_var(&mut self, var: &str) -> Option<String> {
   |                                    ^^^ help: if this is intentional, prefix it with an underscore: `_var`
   |
   = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `proc-macro-srv` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `proc-macro-srv` due to previous error
Build completed unsuccessfully in 0:03:23
