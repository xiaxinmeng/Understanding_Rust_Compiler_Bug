rust
if builder.config.rust_codegen_backends.contains(&INTERNER.intern_str("llvm")) {
    let src_exe = exe("llvm-dwp", target_compiler.host);
    let dst_exe = exe("rust-llvm-dwp", target_compiler.host);
    let llvm_config_bin = builder.ensure(native::Llvm { target: target_compiler.host });
    if !builder.config.dry_run {
        if target_compiler.host == build_compiler.host {
            let llvm_bin_dir = output(Command::new(llvm_config_bin).arg("--bindir"));
            let llvm_bin_dir = Path::new(llvm_bin_dir.trim());
            builder.copy(&llvm_bin_dir.join(&src_exe), &libdir_bin.join(&dst_exe));
        } else {
            let llvm_bin_dir = format!("build/{}/llvm/build/bin", target_compiler.host);
            let llvm_bin_dir = Path::new(&llvm_bin_dir);
            builder.copy(&llvm_bin_dir.join(&src_exe), &libdir_bin.join(&dst_exe));
        }
    }
}
