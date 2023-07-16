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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs at line 559:
                         let assoc_ty_substs = tcx.intern_substs(&substs);
 
                         let bound_vars = tcx.mk_bound_variable_kinds(bound_vars.into_iter());
-                        let bound = EarlyBinder(bound.0.kind().skip_binder()).subst(tcx, assoc_ty_substs);
+                        let bound =
+                            EarlyBinder(bound.0.kind().skip_binder()).subst(tcx, assoc_ty_substs);
                         tcx.mk_predicate(ty::Binder::bind_with_vars(bound, bound_vars))
                 let normalized_bound = normalize_with_depth_to(
                 let normalized_bound = normalize_with_depth_to(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/ascribe_user_type.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/coherence.rs" "/checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
