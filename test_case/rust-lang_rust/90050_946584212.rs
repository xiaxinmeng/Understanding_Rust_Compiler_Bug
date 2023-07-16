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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs at line 480:
     }
 
     if let Some(trait_ref) = trait_ref {
-        let trait_ref = tcx.normalize_erasing_late_bound_regions(ty::ParamEnv::reveal_all(), trait_ref);
+        let trait_ref =
+            tcx.normalize_erasing_late_bound_regions(ty::ParamEnv::reveal_all(), trait_ref);
         push_item_name(tcx, trait_ref.def_id, true, &mut vtable_name);
         visited.clear();
-        push_generic_params_internal(
-            tcx,
-            trait_ref.substs,
-            &mut vtable_name,
-            &mut visited,
-        );
+        push_generic_params_internal(tcx, trait_ref.substs, &mut vtable_name, &mut visited);
     } else {
         vtable_name.push_str("_");
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath.rs" "/checkout/compiler/rustc_codegen_ssa/src/meth.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/debuginfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/coverageinfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/statement.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
