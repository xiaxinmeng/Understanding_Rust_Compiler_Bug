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
Diff in /checkout/compiler/rustc_typeck/src/check/demand.rs at line 158:
         // pointers in typeck, instead of only during borrowck. This can lead
         // to these `RegionsInsufficientlyPolymorphic` errors that aren't helpful.
         if !is_insufficiently_polymorphic {
-            self.emit_coerce_suggestions(&mut err, expr, expr_ty, expected, expected_ty_expr, Some(e));
+            self.emit_coerce_suggestions(
+                expr,
+                expr_ty,
+                expected,
+                expected_ty_expr,
+                expected_ty_expr,
+                Some(e),
+            );
         }
 
         (expected, Some(err))
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/fallback.rs" "/checkout/compiler/rustc_typeck/src/check/compare_method.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_propagate.rs" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_visualize.rs" "/checkout/compiler/rustc_typeck/src/check/demand.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
