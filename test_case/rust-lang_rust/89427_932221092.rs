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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs at line 13:
 use crate::traits::coherence::Conflict;
 use crate::traits::coherence::Conflict;
 use crate::traits::{util, SelectionResult};
-use crate::traits::{Overflow, Unimplemented, Ambiguous};
+use crate::traits::{Ambiguous, Overflow, Unimplemented};
 
 use super::BuiltinImplConditions;
 use super::IntercrateAmbiguityCause;
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs at line 189:
                     // and report ambiguity.
                     if i > 1 {
                         debug!("multiple matches, ambig");
-                        return Err(Ambiguous(candidates.into_iter().filter_map(|c| match c.candidate {
-                            SelectionCandidate::ImplCandidate(def_id) => Some(def_id),
-                            _ => None,
-                        }).collect()));
+                        return Err(Ambiguous(
+                            candidates
+                                .into_iter()
+                                .filter_map(|c| match c.candidate {
+                                    SelectionCandidate::ImplCandidate(def_id) => Some(def_id),
+                                    _ => None,
+                                .collect(),
+                        ));
                     }
                 }
                 }
             }
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs at line 2590:
 }
 
 impl<'o, 'tcx> TraitObligationStackList<'o, 'tcx> {
-    pub fn empty(cache: &'o ProvisionalEvaluationCache<'tcx>) -> TraitObligationStackList<'o, 'tcx> {
+    pub fn empty(
+        cache: &'o ProvisionalEvaluationCache<'tcx>,
+    ) -> TraitObligationStackList<'o, 'tcx> {
         TraitObligationStackList { cache, head: None }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/project.rs" "/checkout/compiler/rustc_trait_selection/src/traits/fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/engine.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
