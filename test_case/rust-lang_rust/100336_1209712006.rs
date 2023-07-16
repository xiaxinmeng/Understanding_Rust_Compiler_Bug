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
Diff in /checkout/compiler/rustc_typeck/src/check/wfcheck.rs at line 1057:
                     field.span,
                     Some(WellFormedLoc::Ty(field.def_id)),
                     field.ty.into(),
+                    None,
                 )
             }
 
 
Diff in /checkout/compiler/rustc_typeck/src/check/wfcheck.rs at line 1214:
         }
 
 
-        wfcx.register_wf_obligation(ty_span, Some(WellFormedLoc::Ty(item_id)), item_ty.into(), None);
+        wfcx.register_wf_obligation(
+            ty_span,
+            Some(WellFormedLoc::Ty(item_id)),
+            item_ty.into(),
+        );
+        );
         if forbid_unsized {
             wfcx.register_bound(
                 traits::ObligationCause::new(ty_span, wfcx.body_id, traits::WellFormed(None)),
Diff in /checkout/compiler/rustc_typeck/src/check/wfcheck.rs at line 1329:
                     // parameter includes another (e.g., `<T, U = T>`). In those cases, we can't
                     // be sure if it will error or not as user might always specify the other.
                     if !ty.needs_subst() {
-                        wfcx.register_wf_obligation(tcx.def_span(param.def_id), None, ty.into(), None);
+                        wfcx.register_wf_obligation(
+                            tcx.def_span(param.def_id),
+                            ty.into(),
+                            None,
+                        );
                     }
                     }
                 }
             }
Diff in /checkout/compiler/rustc_typeck/src/check/wfcheck.rs at line 1534:
     implied_bounds.extend(sig.inputs());
 
     // override the env when checking the return type. `~const` bounds can be fulfilled with non-const implementations.
-    wfcx.register_wf_obligation(hir_decl.output.span(), None, sig.output().into(), Some(hir::Constness::NotConst));
+    wfcx.register_wf_obligation(
+        hir_decl.output.span(),
+        None,
+        sig.output().into(),
+        Some(hir::Constness::NotConst),
 
 
     // FIXME(#27579) return types should not be implied bounds
     implied_bounds.insert(sig.output());
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/diverges.rs" "/checkout/compiler/rustc_typeck/src/check/wfcheck.rs" "/checkout/compiler/rustc_typeck/src/check/rvalue_scopes.rs" "/checkout/compiler/rustc_typeck/src/check/compare_method.rs" "/checkout/compiler/rustc_typeck/src/check/intrinsicck.rs" "/checkout/compiler/rustc_typeck/src/check/op.rs" "/checkout/compiler/rustc_typeck/src/check/expectation.rs" "/checkout/compiler/rustc_typeck/src/check/closure.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
