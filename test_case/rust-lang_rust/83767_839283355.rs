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
Diff in /checkout/compiler/rustc_symbol_mangling/src/v0.rs at line 494:
                         let dummy_self = cx.tcx.mk_ty_infer(ty::FreshTy(0));
                         let trait_ref = trait_ref.with_self_ty(cx.tcx, dummy_self);
                         cx = cx.print_def_path(trait_ref.def_id, trait_ref.substs)?;
-                        while let Some(projection_pred) = predicate_iter
-                            .next_if(|p| matches!(p.skip_binder(), ty::ExistentialPredicate::Projection(_)))
-                        {
+                        while let Some(projection_pred) = predicate_iter.next_if(|p| {
+                            matches!(p.skip_binder(), ty::ExistentialPredicate::Projection(_))
+                        }) {
                             let projection = match projection_pred.skip_binder() {
                                 ty::ExistentialPredicate::Projection(projection) => projection,
                                 _ => unreachable!(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir/src/hir.rs" "/checkout/compiler/rustc_ast/src/ast_like.rs" "/checkout/compiler/rustc_ast/src/attr/mod.rs" "/checkout/compiler/rustc_symbol_mangling/src/test.rs" "/checkout/compiler/rustc_symbol_mangling/src/lib.rs" "/checkout/compiler/rustc_symbol_mangling/src/v0.rs" "/checkout/compiler/rustc_query_system/src/lib.rs" "/checkout/compiler/rustc_ast/src/visit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
