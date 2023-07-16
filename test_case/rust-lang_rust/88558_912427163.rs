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
Diff in /checkout/compiler/rustc_mir/src/transform/check_consts/qualifs.rs at line 119:
             // without having the lang item present.
         };
         };
-        let trait_ref = ty::TraitRef {
-            def_id: drop_trait,
-            substs: cx.tcx.mk_substs_trait(ty, &[]),
+        let trait_ref =
+        let trait_ref =
+            ty::TraitRef { def_id: drop_trait, substs: cx.tcx.mk_substs_trait(ty, &[]) };
         let obligation = Obligation::new(
             ObligationCause::dummy(),
             cx.param_env,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/check_consts/resolver.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/qualifs.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/check.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/mod.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/ops.rs" "/checkout/compiler/rustc_mir/src/transform/const_goto.rs" "/checkout/compiler/rustc_mir/src/transform/nrvo.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
