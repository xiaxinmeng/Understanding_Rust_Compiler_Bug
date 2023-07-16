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
Diff in /checkout/compiler/rustc_lint/src/early.rs at line 124:
         self.with_lint_attrs(f.id, &f.attrs, |cx| {
             ast_visit::walk_expr_field(cx, f);
-
     }
 
 
     fn visit_stmt(&mut self, s: &'a ast::Stmt) {
Diff in /checkout/compiler/rustc_lint/src/early.rs at line 398:
     // that was not lint-checked (perhaps it doesn't exist?). This is a bug.
     for (id, lints) in buffered.map {
         for early_lint in lints {
-            sess.delay_span_bug(early_lint.span, &format!("failed to process buffered lint here (dummy = {})", id == ast::DUMMY_NODE_ID));
+            sess.delay_span_bug(
+                early_lint.span,
+                &format!(
+                    "failed to process buffered lint here (dummy = {})",
+                    id == ast::DUMMY_NODE_ID
+            );
         }
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast/src/util/comments/tests.rs" "/checkout/compiler/rustc_ast/src/attr/mod.rs" "/checkout/compiler/rustc_ast/src/ast.rs" "/checkout/compiler/rustc_lint/src/non_ascii_idents.rs" "/checkout/compiler/rustc_lint/src/traits.rs" "/checkout/compiler/rustc_lint/src/methods.rs" "/checkout/compiler/rustc_lint/src/early.rs" "/checkout/compiler/rustc_lint/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
