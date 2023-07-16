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
Diff in /checkout/compiler/rustc_typeck/src/bounds.rs at line 65:
             })
         });
 
-        self.region_bounds.iter().map(|&(region_bound, span)| {
-                region_bound
-                region_bound
-                    .map_bound(|region_bound| ty::OutlivesPredicate(param_ty, region_bound))
-                    .to_predicate(tcx),
-                span,
-        })
+        self.region_bounds
+            .iter()
+            .iter()
+            .map(|&(region_bound, span)| {
+                    region_bound
+                    region_bound
+                        .map_bound(|region_bound| ty::OutlivesPredicate(param_ty, region_bound))
+                        .to_predicate(tcx),
+                    span,
+            })
+            })
             .chain(self.trait_bounds.iter().map(|&(bound_trait_ref, span, constness)| {
                 let predicate = bound_trait_ref.with_constness(constness).to_predicate(tcx);
                 (predicate, span)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/collect.rs" "/checkout/compiler/rustc_errors/src/annotate_snippet_emitter_writer.rs" "/checkout/compiler/rustc_typeck/src/lib.rs" "/checkout/compiler/rustc_errors/src/emitter.rs" "/checkout/compiler/rustc_typeck/src/bounds.rs" "/checkout/compiler/rustc_errors/src/lock.rs" "/checkout/compiler/rustc_typeck/src/astconv/generics.rs" "/checkout/compiler/rustc_errors/src/diagnostic_builder.rs"` failed.
Build completed unsuccessfully in 0:00:17
Build completed unsuccessfully in 0:00:17
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
