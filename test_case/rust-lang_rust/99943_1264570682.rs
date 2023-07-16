plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/intrinsics.rs at line 2232:
     /// compile-time and at run-time. The unsafe code in crate B is fine.
     #[cfg(not(bootstrap))]
     #[rustc_const_unstable(feature = "const_eval_select", issue = "none")]
-    pub fn const_eval_select<ARG: Tuple, F, G, RET>(arg: ARG, called_in_const: F, called_at_rt: G) -> RET
+    pub fn const_eval_select<ARG: Tuple, F, G, RET>(
+        arg: ARG,
+        called_in_const: F,
+        called_at_rt: G,
     where
     where
         G: FnOnce<ARG, Output = RET>,
         F: FnOnce<ARG, Output = RET>;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/intrinsics.rs" "/checkout/library/core/src/error.rs" "/checkout/library/core/src/async_iter/mod.rs" "/checkout/library/core/src/async_iter/from_iter.rs" "/checkout/library/core/src/async_iter/async_iter.rs" "/checkout/library/core/src/const_closure.rs" "/checkout/library/core/src/slice/iter/macros.rs" "/checkout/library/core/src/panicking.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
