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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs at line 203:
             }
         }
 
-        if self.tcx().features().impl_unified_exhaustive_const_traits
-          && candidates.is_empty() {
-              let mut out = vec![];
-              self.assemble_candidates_from_unified_impls(stack.obligation, &mut out);
-              if !out.is_empty() {
+        if self.tcx().features().impl_unified_exhaustive_const_traits && candidates.is_empty() {
+            let mut out = vec![];
+            self.assemble_candidates_from_unified_impls(stack.obligation, &mut out);
+            if !out.is_empty() {
                 assert!(out.len() == 1);
                 candidates.push(EvaluatedCandidate {
-                  candidate: out.pop().unwrap(),
-                  evaluation: crate::traits::EvaluationResult::EvaluatedToOk,
+                    candidate: out.pop().unwrap(),
+                    evaluation: crate::traits::EvaluationResult::EvaluatedToOk,
-              }
+            }
         }
 
 
         // If there are *NO* candidates, then there are no impls --
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs" "/checkout/compiler/rustc_errors/src/emitter.rs" "/checkout/compiler/rustc_errors/src/diagnostic_impls.rs" "/checkout/compiler/rustc_errors/src/lib.rs" "/checkout/compiler/rustc_trait_selection/src/errors.rs" "/checkout/compiler/rustc_errors/src/translation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/ambiguity.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
