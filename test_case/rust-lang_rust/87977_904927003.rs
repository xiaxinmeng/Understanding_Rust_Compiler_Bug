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
Diff in /checkout/library/unwind/build.rs at line 26:
             // Since ndk r23 beta 3 `libgcc` was replaced with `libunwind` thus
             // check if we have `libunwind` available and if so use it. Otherwise
             // fall back to `libgcc` to support older ndk versions.
-            let has_unwind = build
-                .is_flag_supported("-lunwind")
-                .expect("Unable to invoke compiler");
+            let has_unwind =
+                build.is_flag_supported("-lunwind").expect("Unable to invoke compiler");
             if has_unwind {
             if has_unwind {
                 println!("cargo:rustc-link-lib=unwind");
Diff in /checkout/library/unwind/build.rs at line 66:
 
 mod llvm_libunwind {
 mod llvm_libunwind {
-    use std::{env, io, fs};
     use std::path::{Path, PathBuf};
+    use std::{env, fs, io};
 
     /// Compile the libunwind C/C++ source code.
     pub fn compile() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/vec/splice.rs" "/checkout/library/alloc/src/vec/spec_from_iter_nested.rs" "/checkout/library/alloc/src/vec/mod.rs" "/checkout/library/unwind/build.rs" "/checkout/library/alloc/src/vec/source_iter_marker.rs" "/checkout/library/alloc/src/vec/into_iter.rs" "/checkout/library/alloc/src/vec/drain_filter.rs" "/checkout/library/alloc/src/vec/drain.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
