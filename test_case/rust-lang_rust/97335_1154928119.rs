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
Diff in /checkout/compiler/rustc_resolve/src/def_collector.rs at line 310:
                             self.expansion.to_expn_id(),
                             ty.span,
-                        self.resolver
-                            .impl_trait_context
-                            .impl_trait_context
-                            .insert(def_id, self.impl_trait_context);
+                        self.resolver.impl_trait_context.insert(def_id, self.impl_trait_context);
                     }
                     ImplTraitContext::Existential => {
Diff in /checkout/compiler/rustc_resolve/src/def_collector.rs at line 324:
                     }
                     }
                     ImplTraitContext::ReturnOpaquePositionTy => {
                         let def_id = self.create_def(node_id, DefPathData::ImplTrait, ty.span);
-                            .impl_trait_context
-                            .impl_trait_context
-                            .insert(def_id, self.impl_trait_context);
+                        self.resolver.impl_trait_context.insert(def_id, self.impl_trait_context);
                     }
                 };
Diff in /checkout/compiler/rustc_resolve/src/def_collector.rs at line 404:
Diff in /checkout/compiler/rustc_resolve/src/def_collector.rs at line 404:
                     self.expansion.to_expn_id(),
                     constraint.span,
-                self.resolver
-                    .impl_trait_context
-                    .impl_trait_context
-                    .insert(def_id, self.impl_trait_context);
+                self.resolver.impl_trait_context.insert(def_id, self.impl_trait_context);
         }
 
Diff in /checkout/compiler/rustc_resolve/src/def_collector.rs at line 413:
Diff in /checkout/compiler/rustc_resolve/src/def_collector.rs at line 413:
         if let ImplTraitContext::ReturnOpaquePositionTy = self.impl_trait_context {
             if let AssocConstraintKind::Bound { .. } = constraint.kind {
                 let node_id = constraint.impl_trait_id;
-                let def_id = self.create_def(
-                    node_id,
-                    DefPathData::ImplTrait,
-                    constraint.span,
-                self.resolver
-                    .impl_trait_context
-                    .impl_trait_context
-                    .insert(def_id, self.impl_trait_context);
+                let def_id = self.create_def(node_id, DefPathData::ImplTrait, constraint.span);
+                self.resolver.impl_trait_context.insert(def_id, self.impl_trait_context);
         }
         visit::walk_assoc_constraint(self, constraint);
         visit::walk_assoc_constraint(self, constraint);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/late/lifetimes.rs" "/checkout/compiler/rustc_resolve/src/def_collector.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs" "/checkout/compiler/rustc_passes/src/liveness.rs" "/checkout/compiler/rustc_passes/src/hir_stats.rs" "/checkout/compiler/rustc_resolve/src/check_unused.rs" "/checkout/compiler/rustc_passes/src/dead.rs" "/checkout/compiler/rustc_passes/src/lib_features.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
