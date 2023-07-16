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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_typeck/src/check/cast.rs at line 338:
                 );
                 err.span_label(self.span, "invalid cast");
                 if self.expr_ty.is_numeric() {
-                    err.span_help(self.span, if self.expr_ty == fcx.tcx.types.i8 {
-                        "try casting from `u8` instead"
-                    } else if self.expr_ty == fcx.tcx.types.u32 {
-                        "try `char::from_u32` instead"
-                    } else {
-                        "try `char::from_u32` instead (via a `u32`)"
+                    err.span_help(
+                        self.span,
+                        self.span,
+                        if self.expr_ty == fcx.tcx.types.i8 {
+                            "try casting from `u8` instead"
+                        } else if self.expr_ty == fcx.tcx.types.u32 {
+                            "try `char::from_u32` instead"
+                        } else {
+                            "try `char::from_u32` instead (via a `u32`)"
+                    );
                 }
                 }
                 err.emit();
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_typeck/src/check/regionck.rs" "/checkout/compiler/rustc_typeck/src/check/callee.rs" "/checkout/compiler/rustc_typeck/src/check/gather_locals.rs" "/checkout/compiler/rustc_typeck/src/check/diverges.rs" "/checkout/compiler/rustc_typeck/src/check/expr.rs" "/checkout/compiler/rustc_typeck/src/check/op.rs" "/checkout/compiler/rustc_typeck/src/check/cast.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
