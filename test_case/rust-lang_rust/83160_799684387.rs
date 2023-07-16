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
Diff in /checkout/library/core/src/macros/mod.rs at line 1468:
     #[rustc_builtin_macro]
     #[stable(feature = "rust1", since = "1.0.0")]
     #[allow_internal_unstable(core_intrinsics, libstd_sys_internals)]
-    #[rustc_deprecated(since = "1.52.0", reason = "rustc-serialize is deprecated and no longer supported")]
+    #[rustc_deprecated(
+        since = "1.52.0",
+        reason = "rustc-serialize is deprecated and no longer supported"
+    )]
     pub macro RustcDecodable($item:item) {
     }
Diff in /checkout/library/core/src/macros/mod.rs at line 1477:
     #[rustc_builtin_macro]
     #[rustc_builtin_macro]
     #[stable(feature = "rust1", since = "1.0.0")]
     #[allow_internal_unstable(core_intrinsics)]
-    #[rustc_deprecated(since = "1.52.0", reason = "rustc-serialize is deprecated and no longer supported")]
+    #[rustc_deprecated(
+        since = "1.52.0",
+        reason = "rustc-serialize is deprecated and no longer supported"
+    )]
     pub macro RustcEncodable($item:item) {
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/slice/iter/macros.rs" "/checkout/library/core/src/slice/sort.rs" "/checkout/library/core/src/slice/index.rs" "/checkout/library/core/src/task/poll.rs" "/checkout/src/tools/tier-check/src/main.rs" "/checkout/src/tools/rustdoc/main.rs" "/checkout/src/tools/tidy/src/unit_tests.rs" "/checkout/library/core/src/macros/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
