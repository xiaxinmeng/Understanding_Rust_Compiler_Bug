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
Diff in /checkout/compiler/rustc_codegen_llvm/src/llvm_util.rs at line 273:
         Some(_) | None => {}
 
-
-
     // Features implied by an implicit or explicit `--target`.
-    features.extend(sess
-        .target
-        .features
-        .split(',')
-        .filter(|f| !f.is_empty() && !RUSTC_SPECIFIC_FEATURES.iter().any(|s| f.contains(s)))
-        .map(String::from));
+        sess.target
+            .features
+            .features
+            .split(',')
+            .filter(|f| !f.is_empty() && !RUSTC_SPECIFIC_FEATURES.iter().any(|s| f.contains(s)))
+            .map(String::from),
+    );
     // -Ctarget-features
     features.extend(
     features.extend(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/llvm/ffi.rs" "/checkout/compiler/rustc_codegen_llvm/src/base.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/context.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/archive_ro.rs" "/checkout/compiler/rustc_codegen_llvm/src/type_of.rs" "/checkout/compiler/rustc_codegen_llvm/src/callee.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm_util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
