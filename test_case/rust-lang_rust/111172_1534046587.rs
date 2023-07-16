plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:11f435bab7d0fe51e2f33ae0ce77f457216db8a6)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-d6og6thb/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built d622df9205b2
Successfully tagged rust-ci:latest
Built container sha256:d622df9205b2faa26b616b8899c73b6370b63c732e21acf42b59802eee31bdc4
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281
upload failed: - to s3://rust-lang-ci-sccache2/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 16.62s
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 4:
 #![allow(unused_imports)]
 use crate::infer::canonical::{Canonical, CanonicalVarInfos};
 use crate::infer::region_constraints::MemberConstraint;
-use crate::solve::{Response, ExternalConstraintsData};
+use crate::solve::{ExternalConstraintsData, Response};
 use rustc_middle::ty::{TyCtxt, UniverseIndex};
-use std::ops::Deref;
-use std::ops::Deref;
+use rustc_data_structures::fx::{FxHashMap, FxHashSet, FxIndexMap, FxIndexSet};
 use std::hash::Hash;
-use rustc_data_structures::fx::{FxHashSet, FxHashMap, FxIndexMap, FxIndexSet};
+use std::ops::Deref;
-mod solver;
 mod constraint_walker;
+mod solver;
+mod solver;
 use constraint_walker::{ConstraintWalker, Outlives};
 pub struct Deduper<'tcx> {
 pub struct Deduper<'tcx> {
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 44:
         input.value.external_constraints = tcx.mk_external_constraints(constraints);
     fn dedup_internal(
-        &mut self, 
-        constraints: &mut ExternalConstraintsData<'tcx>, 
+        &mut self,
+        &mut self,
+        constraints: &mut ExternalConstraintsData<'tcx>,
         variables: &mut CanonicalVarInfos<'tcx>,
-        max_universe: &mut UniverseIndex
-    ) {    
+        max_universe: &mut UniverseIndex,
+    ) {
         dedup_exact_eq(&mut constraints.region_constraints.outlives);
         dedup_exact_eq(&mut constraints.region_constraints.member_constraints);
+
+
         let dedupable_vars: FxIndexSet<usize> = variables
             .iter()
             .enumerate()
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 63:
 
         let rule_vars = std::mem::take(&mut self.rule_vars);
         let rule_cats = std::mem::take(&mut self.rule_cats).into_values().collect::<Vec<_>>();
-        let unremovable_vars: FxIndexSet<usize> = (0..variables.len())
-            .filter(|x| !dedupable_vars.contains(x))
-            .collect();
-        
+        let unremovable_vars: FxIndexSet<usize> =
+            (0..variables.len()).filter(|x| !dedupable_vars.contains(x)).collect();
+
         let solve_result = solver::DedupSolver::dedup(rule_vars, rule_cats, unremovable_vars);
         self.remove_duplicate_constraints(&solve_result.removed_constraints, constraints);
         self.compress_variables(&solve_result.removed_vars, constraints, variables, max_universe);
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 74:
     // Extracts data about each constraint, i.e. the variables present, as well as the constraint
     fn extract_constraint_data(
-        &mut self, 
-        &mut self, 
-        dedupable_vars: &FxIndexSet<usize>, 
+        &mut self,
+        dedupable_vars: &FxIndexSet<usize>,
         constraints: &mut ExternalConstraintsData<'tcx>,
-        variables: &mut CanonicalVarInfos<'tcx>
+        variables: &mut CanonicalVarInfos<'tcx>,
     ) {
         let num_vars = variables.len();
         // dummy_var_rewriter is the fetch_var function that will be given to ConstraintWalker
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 97:
             let mut extractor = ConstraintWalker::new(self.tcx, &mut dummy_var_rewriter);
             let erased = ConstraintType::Outlives(extractor.walk_outlives(&outlives.0));
             let vars = std::mem::take(&mut extractor.vars);
-            if vars.is_empty() { continue; }
+            if vars.is_empty() {
+            }
+            }
             self.process_constraint_data(indx, erased, vars);
         }
         for (indx, member) in constraints.region_constraints.member_constraints.iter().enumerate() {
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 104:
             let mut extractor = ConstraintWalker::new(self.tcx, &mut dummy_var_rewriter);
             let erased = ConstraintType::Member(extractor.walk_members(member));
             let vars = std::mem::take(&mut extractor.vars);
-            if vars.is_empty() { continue; }
+            if vars.is_empty() {
+            }
+            }
             self.process_constraint_data(indx, erased, vars);
     }
     }
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 111:
-        &mut self, 
+        &mut self,
         input_indx: usize,
-        erased: ConstraintType<'tcx>, 
-        erased: ConstraintType<'tcx>, 
-        vars: Vec<usize>
+        erased: ConstraintType<'tcx>,
+        vars: Vec<usize>,
     ) {
         self.rule_vars.push(vars);
         let constraint_indx = self.rule_vars.len() - 1;
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 119:
         match &erased {
             ConstraintType::Outlives(_) => &mut self.indx_to_outlives,
             ConstraintType::Member(_) => &mut self.indx_to_members,
-        }.insert(constraint_indx, input_indx);
+        }
+        .insert(constraint_indx, input_indx);
         self.rule_cats.entry(erased).or_insert_with(Vec::new).push(constraint_indx);
     }
-    fn remove_duplicate_constraints(&mut self, to_remove: &FxIndexSet<usize>, constraints: &mut ExternalConstraintsData<'tcx>) {
-        let mut remove_outlives: FxIndexSet<usize> = to_remove.iter().filter_map(|x| self.indx_to_outlives.get(x)).cloned().collect();
-        let mut remove_members: FxIndexSet<usize> = to_remove.iter().filter_map(|x| self.indx_to_members.get(x)).cloned().collect();
+    fn remove_duplicate_constraints(
+        &mut self,
+        to_remove: &FxIndexSet<usize>,
+        constraints: &mut ExternalConstraintsData<'tcx>,
+    ) {
+        let mut remove_outlives: FxIndexSet<usize> =
+            to_remove.iter().filter_map(|x| self.indx_to_outlives.get(x)).cloned().collect();
+        let mut remove_members: FxIndexSet<usize> =
+            to_remove.iter().filter_map(|x| self.indx_to_members.get(x)).cloned().collect();
         remove_outlives.sort();
         remove_members.sort();
 
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 136:
     }
     fn compress_variables(
-        &mut self, 
-        &mut self, 
-        removed_vars: &FxIndexSet<usize>, 
+        &mut self,
+        removed_vars: &FxIndexSet<usize>,
         constraints: &mut ExternalConstraintsData<'tcx>,
         variables: &mut CanonicalVarInfos<'tcx>,
-        max_universe: &mut UniverseIndex
+        max_universe: &mut UniverseIndex,
     ) {
         let mut vars = variables.as_slice().to_vec();
-        let mut universes_available: FxIndexSet<UniverseIndex> = vars.iter().map(|x| x.universe()).collect();
+        let mut universes_available: FxIndexSet<UniverseIndex> =
+            vars.iter().map(|x| x.universe()).collect();
         universes_available.sort();
 
         let mut compressed_vars: FxHashMap<usize, usize> = FxHashMap::default();
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 166:
 
         for var in vars.iter_mut() {
             *var = var.with_updated_universe(
-                *universes_available.get_index(
-                    universes_used.get_index_of(&var.universe()).unwrap()
+                *universes_available
+                *universes_available
+                    .get_index(universes_used.get_index_of(&var.universe()).unwrap())
+                    .unwrap(),
         }
 
 
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 190:
 fn dedup_exact_eq<T: PartialEq>(input: &mut Vec<T>) {
     let mut indx = 0;
     while indx < input.len() {
-        if input.iter().skip(indx+1).any(|x| x == &input[indx]) {
+        if input.iter().skip(indx + 1).any(|x| x == &input[indx]) {
             input.swap_remove(indx);
         }
         }
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs at line 197:
         indx += 1;
 }
+
+
Diff in /checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/solver.rs at line 90:
             }
             removed_vars.insert(*from);
-        DedupResult {
-            removed_constraints: deduper.removed_rules.into_inner(),
-            removed_vars,
-        }
-        }
+        DedupResult { removed_constraints: deduper.removed_rules.into_inner(), removed_vars }
     fn refine_categories(&mut self) {
         // Refine categories based on shape
         // Refine categories based on shape
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/lib.rs" "/checkout/compiler/rustc_trait_selection/src/solve/assembly/structural_traits.rs" "/checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/constraint_walker.rs" "/checkout/compiler/rustc_trait_selection/src/solve/assembly/mod.rs" "/checkout/compiler/rustc_trait_selection/src/solve/canonicalize.rs" "/checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/mod.rs" "/checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical/dedup_solver/solver.rs" "/checkout/compiler/rustc_trait_selection/src/solve/eval_ctxt/canonical.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
