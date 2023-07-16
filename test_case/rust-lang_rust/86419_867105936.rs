plain
Successfully built ad5ef217f0ff
Successfully tagged rust-ci:latest
Built container sha256:ad5ef217f0fffcd74557f8d52f2833e59702db59656f36546341974c47b16bb7
Uploading finished image to https://ci-caches.rust-lang.org/docker/3ccdc63d9d4b09f940536f268273cee5fff0a2be1dc7e629c4be003d524f779b5660e37b0d2b307b5801865dae2862c25a4c07afffb0a92ab35aed41de335d50
upload failed: - to s3://rust-lang-ci-sccache2/docker/3ccdc63d9d4b09f940536f268273cee5fff0a2be1dc7e629c4be003d524f779b5660e37b0d2b307b5801865dae2862c25a4c07afffb0a92ab35aed41de335d50 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
Diff in /checkout/compiler/rustc_codegen_llvm/src/back/archive.rs at line 206:
         // All import names are Rust identifiers and therefore cannot contain \0 characters.
         // FIXME: when support for #[link_name] implemented, ensure that import.name values don't
         // have any \0 characters
-        let import_name_vector: Vec<CString> =
-            if self.config.sess.target.arch == "x86" {
-                dll_imports.iter().map(LlvmArchiveBuilder::i686_decorated_name).collect()
-                dll_imports
-                    .iter()
-                    .iter()
-                    .map(|import: &DllImport| CString::new(import.name.to_string()).unwrap())
-                    .collect()
-            };
+        let import_name_vector: Vec<CString> = if self.config.sess.target.arch == "x86" {
+            dll_imports.iter().map(LlvmArchiveBuilder::i686_decorated_name).collect()
+            dll_imports
+                .iter()
+                .iter()
+                .map(|import: &DllImport| CString::new(import.name.to_string()).unwrap())
+                .collect()
 
 
         let output_path_z = rustc_fs_util::path_to_c_string(&output_path);
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/test/src/stats.rs" "/checkout/compiler/rustc_codegen_llvm/src/type_of.rs" "/checkout/library/test/src/types.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/archive.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/lto.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/profiling.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/write.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/utils.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
