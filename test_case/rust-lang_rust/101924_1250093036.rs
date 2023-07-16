plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_traits/src/type_op.rs at line 93:
         T: TypeFoldable<'tcx>,
         self.infcx
         self.infcx
-            .partially_normalize_associated_types_in(
-                cause,
-                self.param_env,
-            )
-            )
+            .partially_normalize_associated_types_in(cause, self.param_env, value)
             .into_value_registering_obligations(self.infcx, self.fulfill_cx)
 
Diff in /checkout/compiler/rustc_traits/src/type_op.rs at line 167:
                 hir::CRATE_HIR_ID,
                 hir::CRATE_HIR_ID,
                 ObligationCauseCode::AscribeUserTypeProvePredicate(predicate_span),
             );
-            let instantiated_predicate = self.normalize_with_cause(instantiated_predicate, cause.clone());
+            let instantiated_predicate =
+                self.normalize_with_cause(instantiated_predicate, cause.clone());
             self.prove_predicate(instantiated_predicate, cause);
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_traits/src/type_op.rs" "/checkout/compiler/rustc_traits/src/implied_outlives_bounds.rs" "/checkout/compiler/rustc_traits/src/lib.rs" "/checkout/compiler/rustc_traits/src/chalk/db.rs" "/checkout/compiler/rustc_traits/src/normalize_projection_ty.rs" "/checkout/compiler/rustc_traits/src/chalk/lowering.rs" "/checkout/compiler/rustc_traits/src/normalize_erasing_regions.rs" "/checkout/compiler/rustc_traits/src/dropck_outlives.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
