plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs at line 80:
 
         if coverage_mapping_buffer.is_empty() {
             if function_coverage.is_used() {
-                bug!("A used function should have had coverage mapping data but did not: {}", mangled_function_name);
+                bug!(
+                    "A used function should have had coverage mapping data but did not: {}",
+                    mangled_function_name
             } else {
             } else {
                 debug!("unused function had no coverage mapping data: {}", mangled_function_name);
                 continue;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_cranelift/src/inline_asm.rs" "/checkout/compiler/rustc_codegen_cranelift/src/constant.rs" "/checkout/compiler/rustc_codegen_cranelift/src/abi/comments.rs" "/checkout/compiler/rustc_codegen_cranelift/src/debuginfo/unwind.rs" "/checkout/compiler/rustc_codegen_cranelift/src/intrinsics/cpuid.rs" "/checkout/compiler/rustc_codegen_cranelift/src/bin/cg_clif.rs" "/checkout/compiler/rustc_codegen_cranelift/src/driver/jit.rs" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
