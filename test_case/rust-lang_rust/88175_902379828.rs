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
Diff in /checkout/compiler/rustc_ast_lowering/src/expr.rs at line 92:
                 ExprKind::If(ref cond, ref then, ref else_opt) => {
                     self.lower_expr_if(cond, then, else_opt.as_deref())
                 }
-                ExprKind::While(ref cond, ref body, opt_label) => self
-                    .with_loop_scope(e.id, |this| {
-                        let span = this.mark_span_with_reason(DesugaringKind::WhileLoop, e.span, None);
+                ExprKind::While(ref cond, ref body, opt_label) => {
+                    self.with_loop_scope(e.id, |this| {
+                        let span =
+                            this.mark_span_with_reason(DesugaringKind::WhileLoop, e.span, None);
                         this.lower_expr_while_in_loop_scope(span, cond, body, opt_label)
+                    })
+                }
+                }
                 ExprKind::Loop(ref body, opt_label) => self.with_loop_scope(e.id, |this| {
                     hir::ExprKind::Loop(
                         this.lower_block(body, false),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_lowering/src/item.rs" "/checkout/compiler/rustc_index/src/bit_set/tests.rs" "/checkout/compiler/rustc_ast_lowering/src/path.rs" "/checkout/compiler/rustc_ast_lowering/src/expr.rs" "/checkout/compiler/rustc_ast_lowering/src/lib.rs" "/checkout/compiler/rustc_index/src/vec/tests.rs" "/checkout/compiler/rustc_lint/src/levels.rs" "/checkout/compiler/rustc_index/src/vec.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
