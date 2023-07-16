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
Diff in /checkout/compiler/rustc_typeck/src/collect.rs at line 2438:
     bound_vars: &'tcx ty::List<ty::BoundVariableKind>,
 ) -> Vec<(ty::Predicate<'tcx>, Span)> {
     let mut bounds = Bounds::default();
-    astconv.add_bounds(
-        param_ty,
-        std::array::IntoIter::new([bound]),
-        &mut bounds,
-        bound_vars,
-    );
+    astconv.add_bounds(param_ty, std::array::IntoIter::new([bound]), &mut bounds, bound_vars);
     bounds.predicates(astconv.tcx(), param_ty)
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc/src/main.rs" "/checkout/compiler/rustc_typeck/src/outlives/implicit_infer.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs" "/checkout/compiler/rustc_typeck/src/collect.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check.rs" "/checkout/compiler/rustc_typeck/src/structured_errors.rs" "/checkout/compiler/rustc_typeck/src/hir_wf_check.rs" "/checkout/library/unwind/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
