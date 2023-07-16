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
Diff in /checkout/compiler/rustc_lint/src/builtin.rs at line 538:
         let attrs = cx.tcx.hir().attrs(id);
         let has_doc = attrs.iter().any(|a| has_doc(cx.sess(), a));
         if !has_doc {
-            cx.struct_span_lint(
-                MISSING_DOCS,
-                cx.tcx.source_map(()).guess_head_span(sp),
-                |lint| {
-                    lint.build(&format!("missing documentation for {} {}", article, desc)).emit()
-            );
-            );
+            cx.struct_span_lint(MISSING_DOCS, cx.tcx.source_map(()).guess_head_span(sp), |lint| {
+                lint.build(&format!("missing documentation for {} {}", article, desc)).emit()
+            });
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/array_into_iter.rs" "/checkout/compiler/rustc_lint/src/nonstandard_style/tests.rs" "/checkout/compiler/rustc_lint/src/unused.rs" "/checkout/compiler/rustc_lint/src/redundant_semicolon.rs" "/checkout/compiler/rustc_lint/src/early.rs" "/checkout/compiler/rustc_arena/src/lib.rs" "/checkout/compiler/rustc_lint/src/methods.rs" "/checkout/compiler/rustc_lint/src/builtin.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
