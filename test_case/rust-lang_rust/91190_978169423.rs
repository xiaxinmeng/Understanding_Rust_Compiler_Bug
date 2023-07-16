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
Diff in /checkout/compiler/rustc_codegen_llvm/src/back/write.rs at line 400:
 ) -> bool {
     // The new pass manager is causing significant performance issues such as #91128, and is
     // therefore disabled in stable versions of rustc by default.
-    config
-        .new_llvm_pass_manager
-        .unwrap_or(false)
+    config.new_llvm_pass_manager.unwrap_or(false)
 
 
 pub(crate) unsafe fn optimize_with_new_llvm_pass_manager(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/value.rs" "/checkout/compiler/rustc_codegen_llvm/src/lib.rs" "/checkout/compiler/rustc_codegen_llvm/src/base.rs" "/checkout/compiler/rustc_codegen_llvm/src/abi.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/profiling.rs" "/checkout/compiler/rustc_codegen_llvm/src/consts.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/write.rs" "/checkout/compiler/rustc_codegen_llvm/src/type_of.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
