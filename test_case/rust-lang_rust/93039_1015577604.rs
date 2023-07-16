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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_typeck/src/check/expr.rs at line 964:
         }
 
         let result_ty = coerce.complete(self);
-        if cond_ty.references_error() {
-            self.tcx.ty_error()
-            result_ty
-        }
-        }
+        if cond_ty.references_error() { self.tcx.ty_error() } else { result_ty }
 
 
     /// Type check assignment expression `expr` of form `lhs = rhs`.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/demand.rs" "/checkout/compiler/rustc_typeck/src/check/autoderef.rs" "/checkout/compiler/rustc_typeck/src/collect.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs" "/checkout/compiler/rustc_typeck/src/expr_use_visitor.rs" "/checkout/compiler/rustc_typeck/src/collect/item_bounds.rs" "/checkout/compiler/rustc_typeck/src/structured_errors.rs" "/checkout/compiler/rustc_typeck/src/check/expr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
