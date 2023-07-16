
[gulf:~/dev/rust1] grep -r -s -A1 --include=Cargo.lock "name = .*smallvec"
Cargo.lock:name = "smallvec"
Cargo.lock-version = "1.8.1"
--
compiler/rustc_codegen_cranelift/Cargo.lock:name = "smallvec"
compiler/rustc_codegen_cranelift/Cargo.lock-version = "1.8.1"
--
src/tools/miri/Cargo.lock:name = "smallvec"
src/tools/miri/Cargo.lock-version = "1.7.0"
--
src/tools/rust-analyzer/Cargo.lock:name = "smallvec"
src/tools/rust-analyzer/Cargo.lock-version = "1.8.0"
--
src/tools/rls/Cargo.lock:name = "smallvec"
src/tools/rls/Cargo.lock-version = "1.7.0"
