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
Diff in /checkout/compiler/rustc_codegen_llvm/src/back/write.rs at line 56:
     file_type: llvm::FileType,
     self_profiler_ref: &SelfProfilerRef,
 ) -> Result<(), FatalError> {
-    debug!(
-        "write_output_file output={:?} dwo_output={:?}",
-        output, dwo_output
-    );
+    debug!("write_output_file output={:?} dwo_output={:?}", output, dwo_output);
     unsafe {
         let output_c = path_to_c_string(output);
         let dwo_output_c;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/type_.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/lto.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/write.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/profiling.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/archive.rs" "/checkout/compiler/rustc_codegen_llvm/src/builder.rs" "/checkout/compiler/rustc_codegen_llvm/src/declare.rs" "/checkout/compiler/rustc_codegen_llvm/src/allocator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
