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
Diff in /checkout/compiler/rustc_typeck/src/check/method/mod.rs at line 141:
         method_name: Ident,
         self_ty: Ty<'tcx>,
         call_expr: &hir::Expr<'_>,
-        span: Option<Span>
+        span: Option<Span>,
     ) {
         let params = self
             .probe_for_name(
Diff in /checkout/compiler/rustc_typeck/src/check/expr.rs at line 1844:
                 expr_t,
                 expr,
-                None
+                None,
+                None,
             );
         }
         err.emit();
Diff in /checkout/compiler/rustc_typeck/src/check/expr.rs at line 1897:
                 expr_t,
                 expr,
-                Some(span)
+                Some(span),
+                Some(span),
             );
         } else {
             err.help("methods are immutable and cannot be assigned to");
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/generator_interior.rs" "/checkout/compiler/rustc_typeck/src/check/autoderef.rs" "/checkout/compiler/rustc_typeck/src/check/method/suggest.rs" "/checkout/compiler/rustc_typeck/src/check/method/mod.rs" "/checkout/compiler/rustc_typeck/src/check/method/confirm.rs" "/checkout/compiler/rustc_typeck/src/check/method/prelude2021.rs" "/checkout/compiler/rustc_typeck/src/check/method/probe.rs" "/checkout/compiler/rustc_typeck/src/check/place_op.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
