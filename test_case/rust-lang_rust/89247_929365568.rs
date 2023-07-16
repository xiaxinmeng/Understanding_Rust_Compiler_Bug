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
Diff in /checkout/library/core/src/intrinsics.rs at line 818:
     // Although it is const stable, it should never be exposed outside core::intrinsics,
     // and never stabilized.
     #[rustc_const_stable(feature = "const_eval_select", since = "1.57")]
-    pub fn const_eval_select<ARG, F: Fn(ARG) -> RET, G: Fn(ARG) -> RET, RET>(arg: ARG, called_in_const: F, called_at_rt: G) -> RET;
+    pub fn const_eval_select<ARG, F: Fn(ARG) -> RET, G: Fn(ARG) -> RET, RET>(
+        arg: ARG,
+        called_in_const: F,
+        called_at_rt: G,
+    ) -> RET;
     /// The size of a type in bytes.
     ///
     ///
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/stream/from_iter.rs" "/checkout/library/core/src/stream/mod.rs" "/checkout/library/core/src/ptr/metadata.rs" "/checkout/library/core/src/primitive_docs.rs" "/checkout/library/core/src/ptr/non_null.rs" "/checkout/library/core/src/intrinsics.rs" "/checkout/library/core/src/cmp.rs" "/checkout/library/core/src/stream/stream/mod.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
