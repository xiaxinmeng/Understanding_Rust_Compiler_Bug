
diff --git a/compiler/rustc_middle/src/ty/relate.rs b/compiler/rustc_middle/src/ty/relate.rs
index b1cfa3fa964..ff22d1d1902 100644
--- a/compiler/rustc_middle/src/ty/relate.rs
+++ b/compiler/rustc_middle/src/ty/relate.rs
@@ -7,7 +7,7 @@
 use crate::mir::interpret::{get_slice_bytes, ConstValue, GlobalAlloc, Scalar};
 use crate::ty::error::{ExpectedFound, TypeError};
 use crate::ty::subst::{GenericArg, GenericArgKind, Subst, SubstsRef};
-use crate::ty::{self, ImplSubject, Term, Ty, TyCtxt, TypeFoldable};
+use crate::ty::{self, ImplSubject, List, Term, Ty, TyCtxt, TypeFoldable};
 use rustc_hir as ast;
 use rustc_hir::def_id::DefId;
 use rustc_span::DUMMY_SP;
@@ -141,8 +141,11 @@ pub fn relate_substs<'tcx, R: TypeRelation<'tcx>>(
     a_subst: SubstsRef<'tcx>,
     b_subst: SubstsRef<'tcx>,
 ) -> RelateResult<'tcx, SubstsRef<'tcx>> {
-    let tcx = relation.tcx();
+    if a_subst.is_empty() || b_subst.is_empty() {
+        return Ok(List::empty());
+    }
 
+    let tcx = relation.tcx();
     let zipped = iter::zip(a_subst, b_subst);
     match variances {
         Some((ty_def_id, variances)) => {
