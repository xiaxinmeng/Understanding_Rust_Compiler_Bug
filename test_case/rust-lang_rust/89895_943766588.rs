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
Diff in /checkout/compiler/rustc_ast_lowering/src/expr.rs at line 1333:
     ) -> hir::Expr<'hir> {
         // expand <head>
         let head = self.lower_expr_mut(head);
-        let desugared_span = self.mark_span_with_reason(
-            DesugaringKind::ForLoop(ForLoopLoc::Head),
-            head.span,
-        );
+        let desugared_span =
+        let desugared_span =
+            self.mark_span_with_reason(DesugaringKind::ForLoop(ForLoopLoc::Head), head.span, None);
 
         let iter = Ident::with_dummy_span(sym::iter);
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_lowering/src/asm.rs" "/checkout/compiler/rustc_ast_lowering/src/lib.rs" "/checkout/compiler/rustc_infer/src/traits/engine.rs" "/checkout/compiler/rustc_ast_lowering/src/expr.rs" "/checkout/compiler/rustc_infer/src/traits/util.rs" "/checkout/compiler/rustc_ast_lowering/src/pat.rs" "/checkout/compiler/rustc_infer/src/traits/project.rs" "/checkout/compiler/rustc_infer/src/traits/structural_impls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
