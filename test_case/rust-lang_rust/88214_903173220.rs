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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_ast_lowering/src/expr.rs at line 1403:
 
         // #82462: to correctly diagnose borrow errors, the block that contains
         // the iter expr needs to have a span that covers the loop body.
-        let desugared_full_span = self.mark_span_with_reason(
-            DesugaringKind::ForLoop(ForLoopLoc::Head),
-            e.span,
-        );
-        );
+        let desugared_full_span =
+            self.mark_span_with_reason(DesugaringKind::ForLoop(ForLoopLoc::Head), e.span, None);
 
         let match_expr = self.arena.alloc(self.expr_match(
             desugared_full_span,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_lowering/src/lib.rs" "/checkout/compiler/rustc_attr/src/builtin.rs" "/checkout/compiler/rustc_ast_lowering/src/path.rs" "/checkout/compiler/rustc_ast_lowering/src/expr.rs" "/checkout/compiler/rustc_ast_lowering/src/asm.rs" "/checkout/compiler/rustc_attr/src/lib.rs" "/checkout/compiler/rustc_arena/src/lib.rs" "/checkout/compiler/rustc_ast_lowering/src/pat.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
