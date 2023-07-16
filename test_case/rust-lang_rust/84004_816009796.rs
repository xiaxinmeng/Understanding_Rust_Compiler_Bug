plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/compiler/rustc_codegen_llvm/src/llvm/ffi.rs at line 2133:
         T: &TargetMachine,
         Index: size_t,
         Feature: &mut *const c_char,
-        Desc: &mut *const c_char);
+        Desc: &mut *const c_char,
+    );
 
     pub fn LLVMRustGetHostCPUName(len: *mut usize) -> *const c_char;
     pub fn LLVMRustCreateTargetMachine(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/llvm/archive_ro.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/ffi.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/diagnostic.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/namespace.rs" "/checkout/compiler/rustc_codegen_llvm/src/attributes.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/gdb.rs" "/checkout/compiler/rustc_codegen_llvm/src/abi.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:22
