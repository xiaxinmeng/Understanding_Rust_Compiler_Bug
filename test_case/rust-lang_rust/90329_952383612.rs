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
Diff in /checkout/compiler/rustc_typeck/src/check/method/probe.rs at line 1125:
                     .unwrap_or_else(|_| {
                         span_bug!(self.span, "{:?} was applicable but now isn't?", step.self_ty)
                     });
-                self.pick_by_value_method(step, self_ty, Some(&mut unstable_candidates)).or_else(|| {
-                    self.pick_autorefd_method(
-                        self_ty,
-                        hir::Mutability::Not,
-                        Some(&mut unstable_candidates),
-                    )
-                    )
-                    .or_else(|| {
+                self.pick_by_value_method(step, self_ty, Some(&mut unstable_candidates)).or_else(
+                    || {
                         self.pick_autorefd_method(
                             self_ty,
                             self_ty,
Diff in /checkout/compiler/rustc_typeck/src/check/method/probe.rs at line 1139:
-                            hir::Mutability::Mut,
+                            hir::Mutability::Not,
                             Some(&mut unstable_candidates),
-                    })
-                    })
-                    .or_else(|| self.pick_const_ptr_method(step, self_ty, Some(&mut unstable_candidates)))
-                })
+                        .or_else(|| {
+                            self.pick_autorefd_method(
+                                self_ty,
+                                hir::Mutability::Mut,
+                                Some(&mut unstable_candidates),
+                            )
+                            )
+                        })
+                        .or_else(|| {
+                            self.pick_const_ptr_method(
+                                self_ty,
+                                Some(&mut unstable_candidates),
+                            )
+                        })
+                        })
+                    },
+                )
             })
             .next();
 
Diff in /checkout/compiler/rustc_typeck/src/check/method/probe.rs at line 1316:
         unstable_candidates: Option<&mut Vec<(Candidate<'tcx>, Symbol)>>,
     ) -> Option<PickResult<'tcx>>
     where
-        ProbesIter: Iterator<Item = &'b Candidate<'tcx>> + Clone, 'tcx: 'b
+        ProbesIter: Iterator<Item = &'b Candidate<'tcx>> + Clone,
+        'tcx: 'b,
     {
         let mut applicable_candidates: Vec<_> = probes
             .clone()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_typeck/src/check/coercion.rs" "/checkout/compiler/rustc_typeck/src/check/compare_method.rs" "/checkout/compiler/rustc_typeck/src/check/method/probe.rs" "/checkout/compiler/rustc_typeck/src/check/cast.rs" "/checkout/compiler/rustc_typeck/src/check/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
