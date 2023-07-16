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
Diff in /checkout/compiler/rustc_typeck/src/check/wfcheck.rs at line 172:
             for_id(tcx, item.hir_id(), ty_span).with_fcx(|fcx, tcx| {
                 let ty = tcx.type_of(def_id);
                 let item_ty = fcx.normalize_associated_types_in(ty_span, ty);
-                fcx.register_wf_obligation(item_ty.into(), ty_span, ObligationCauseCode::MiscObligation);
+                fcx.register_wf_obligation(
+                    item_ty.into(),
+                    ty_span,
+                    ObligationCauseCode::MiscObligation,
+                );
                 vec![]
             });
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/dropck.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs" "/checkout/compiler/rustc_typeck/src/check/cast.rs" "/checkout/compiler/rustc_typeck/src/check/demand.rs" "/checkout/compiler/rustc_typeck/src/check/wfcheck.rs" "/checkout/compiler/rustc_typeck/src/check/op.rs" "/checkout/compiler/rustc_typeck/src/check/place_op.rs" "/checkout/compiler/rustc_typeck/src/check/_match.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
