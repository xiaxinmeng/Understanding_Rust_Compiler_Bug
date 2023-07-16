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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs at line 284:
                 // structs and enums.
                 self.assemble_candidates_from_impls(obligation, &mut candidates);
 
-                            // For other types, we'll use the builtin rules.
-            let copy_conditions = self.copy_clone_conditions(obligation);
-            self.assemble_builtin_bound_candidates(copy_conditions, &mut candidates);
-        } else if lang_items.discriminant_kind_trait() == Some(def_id) {
-            // `DiscriminantKind` is automatically implemented for every type.
-            candidates.vec.push(DiscriminantKindCandidate);
-        } else if lang_items.pointee_trait() == Some(def_id) {
-            // `Pointee` is automatically implemented for every type.
-            candidates.vec.push(PointeeCandidate);
-        } else if lang_items.sized_trait() == Some(def_id) {
-            // Sized is never implementable by end-users, it is
-            // always automatically computed.
-            let sized_conditions = self.sized_conditions(obligation);
-            self.assemble_builtin_bound_candidates(sized_conditions, &mut candidates);
-        } else if lang_items.unsize_trait() == Some(def_id) {
-            self.assemble_candidates_for_unsizing(obligation, &mut candidates);
-        } else if lang_items.drop_trait() == Some(def_id)
-            && obligation.predicate.skip_binder().constness == ty::BoundConstness::ConstIfConst
-        {
-            if self.is_in_const_context {
-                self.assemble_const_drop_candidates(obligation, stack, &mut candidates)?;
-            } else {
-                debug!("passing ~const Drop bound; in non-const context");
-                // `~const Drop` when we are not in a const context has no effect.
-                candidates.vec.push(ConstDropCandidate)
-            }
+                // For other types, we'll use the builtin rules.
+                let copy_conditions = self.copy_clone_conditions(obligation);
+                self.assemble_builtin_bound_candidates(copy_conditions, &mut candidates);
+            } else if lang_items.discriminant_kind_trait() == Some(def_id) {
+                // `DiscriminantKind` is automatically implemented for every type.
+                candidates.vec.push(DiscriminantKindCandidate);
+            } else if lang_items.pointee_trait() == Some(def_id) {
+                // `Pointee` is automatically implemented for every type.
+                candidates.vec.push(PointeeCandidate);
+            } else if lang_items.sized_trait() == Some(def_id) {
+                // Sized is never implementable by end-users, it is
+                // always automatically computed.
+                let sized_conditions = self.sized_conditions(obligation);
+                self.assemble_builtin_bound_candidates(sized_conditions, &mut candidates);
+            } else if lang_items.unsize_trait() == Some(def_id) {
+                self.assemble_candidates_for_unsizing(obligation, &mut candidates);
+            } else if lang_items.drop_trait() == Some(def_id)
+                && obligation.predicate.skip_binder().constness == ty::BoundConstness::ConstIfConst
+            {
+                if self.is_in_const_context {
+                    self.assemble_const_drop_candidates(obligation, stack, &mut candidates)?;
+                } else {
+                    debug!("passing ~const Drop bound; in non-const context");
+                    // `~const Drop` when we are not in a const context has no effect.
+                    candidates.vec.push(ConstDropCandidate)
             } else {
             } else {
                 if lang_items.clone_trait() == Some(def_id) {
                     // Same builtin conditions as `Copy`, i.e., every type which has builtin support
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
