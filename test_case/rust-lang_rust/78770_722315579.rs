rust
// compiler/rustc_codegen_ssa/src/back/link.rs:765
if let Err(e) = Command::new("dsymutil").arg(out_filename).output() {
    sess.fatal(&format!("failed to run dsymutil: {}", e))
}
