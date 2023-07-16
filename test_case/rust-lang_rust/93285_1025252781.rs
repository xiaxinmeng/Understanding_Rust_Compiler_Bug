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
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 1246:
                 // `<T as Iterator>::Item = u32`
                 let def_kind = tcx.def_kind(projection_ty.skip_binder().item_def_id);
                 match (def_kind, term) {
-                  (hir::def::DefKind::AssocTy, ty::Term::Ty(_)) |
-                  (hir::def::DefKind::AssocConst, ty::Term::Const(_)) => (),
-                  (_, _) => {
-                      tcx.sess
-                          .struct_span_err(binding.span, "type/const mismatch in equality bind of associated field")
-                          .span_label(binding.span, "type/const Mismatch")
-                          .emit();
-                  },
+                    (hir::def::DefKind::AssocTy, ty::Term::Ty(_))
+                    | (hir::def::DefKind::AssocConst, ty::Term::Const(_)) => (),
+                    (_, _) => {
+                        tcx.sess
+                            .struct_span_err(
+                                binding.span,
+                                "type/const mismatch in equality bind of associated field",
+                            )
+                            .span_label(binding.span, "type/const Mismatch")
+                            .emit();
                 }
                 bounds.projection_bounds.push((
                 bounds.projection_bounds.push((
                     projection_ty.map_bound(|projection_ty| ty::ProjectionPredicate {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/variance/constraints.rs" "/checkout/compiler/rustc_typeck/src/variance/xform.rs" "/checkout/compiler/rustc_typeck/src/structured_errors.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check.rs" "/checkout/compiler/rustc_typeck/src/astconv/generics.rs" "/checkout/compiler/rustc_typeck/src/variance/mod.rs" "/checkout/compiler/rustc_typeck/src/astconv/mod.rs" "/checkout/compiler/rustc_typeck/src/variance/test.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
