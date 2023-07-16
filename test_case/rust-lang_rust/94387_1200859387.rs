plain
error[E0046]: not all trait items implemented, missing: `injected_env_var`
  --> crates/proc-macro-srv/src/abis/abi_sysroot/ra_server.rs:74:1
   |
74 | impl server::FreeFunctions for RustAnalyzer {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `injected_env_var` in implementation
   |
   = help: implement the missing item: `fn injected_env_var(&mut self, _: &str) -> Option<std::string::String> { todo!() }`
    Checking hir-def v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-def)
For more information about this error, try `rustc --explain E0046`.
error: could not compile `proc-macro-srv` due to previous error
warning: build failed, waiting for other jobs to finish...
