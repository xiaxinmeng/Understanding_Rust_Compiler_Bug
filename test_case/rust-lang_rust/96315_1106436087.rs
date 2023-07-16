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
Diff in /checkout/compiler/rustc_borrowck/src/constraints/graph.rs at line 225:
     }
 }
 
-impl<'s, 'tcx, D: ConstraintGraphDirecton> graph::GraphSuccessors<'_>
-    for RegionGraph<'s, 'tcx, D>
-{
+impl<'s, 'tcx, D: ConstraintGraphDirecton> graph::GraphSuccessors<'_> for RegionGraph<'s, 'tcx, D> {
     type Item = RegionVid;
     type Iter = Successors<'s, 'tcx, D>;
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/structured_errors/wrong_number_of_generic_args.rs" "/checkout/compiler/rustc_borrowck/src/constraints/graph.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/missing_cast_for_variadic_arg.rs" "/checkout/compiler/rustc_typeck/src/hir_wf_check.rs" "/checkout/compiler/rustc_borrowck/src/place_ext.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs" "/checkout/compiler/rustc_borrowck/src/type_check/relate_tys.rs" "/checkout/compiler/rustc_borrowck/src/constraints/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
