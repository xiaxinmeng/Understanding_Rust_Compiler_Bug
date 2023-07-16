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
Diff in /checkout/compiler/rustc_resolve/src/diagnostics.rs at line 718:
                         );
                     }
                 }
-                Scope::BuiltinTypes => suggestions.extend(PrimTy::ALL.iter().filter_map(|prim_ty| {
-                    let res = Res::PrimTy(*prim_ty);
-                    filter_fn(res).then_some(TypoSuggestion::from_res(prim_ty.name(), res))
-                })),
+                Scope::BuiltinTypes => {
+                    suggestions.extend(PrimTy::ALL.iter().filter_map(|prim_ty| {
+                        let res = Res::PrimTy(*prim_ty);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/diagnostics.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                        filter_fn(res).then_some(TypoSuggestion::from_res(prim_ty.name(), res))
+                    }))
             }
 
             None::<()>
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
