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
Diff in /checkout/compiler/rustc_lint/src/unused.rs at line 477:
         lhs_needs_parens
             || (followed_by_block
                 && match &inner.kind {
-                    ExprKind::Ret(_)
-                    | ExprKind::Break(..)
-                    | ExprKind::Yield(..) => true,
+                    ExprKind::Ret(_) | ExprKind::Break(..) | ExprKind::Yield(..) => true,
                     ExprKind::Range(_lhs, Some(rhs), _limits) => {
                         !classify::expr_requires_semi_to_be_stmt(&rhs)
                     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/types.rs" "/checkout/compiler/rustc_lint/src/early.rs" "/checkout/compiler/rustc_lint/src/noop_method_call.rs" "/checkout/compiler/rustc_lint/src/unused.rs" "/checkout/compiler/rustc_lint/src/lib.rs" "/checkout/compiler/rustc_lint/src/methods.rs" "/checkout/compiler/rustc_lint/src/array_into_iter.rs" "/checkout/compiler/rustc_lint/src/hidden_unicode_codepoints.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
