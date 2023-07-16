diff
diff --git a/compiler/rustc_typeck/src/check/method/probe.rs b/compiler/rustc_typeck/src/check/method/probe.rs
index d4631c465a3..b0febd5eccb 100644
--- a/compiler/rustc_typeck/src/check/method/probe.rs
+++ b/compiler/rustc_typeck/src/check/method/probe.rs
@@ -20,7 +20,7 @@
 use rustc_infer::infer::{self, InferOk, TyCtxtInferExt};
 use rustc_middle::middle::stability;
 use rustc_middle::ty::subst::{InternalSubsts, Subst, SubstsRef};
-use rustc_middle::ty::GenericParamDefKind;
+use rustc_middle::ty::{GenericParamDefKind, TypeAndMut};
 use rustc_middle::ty::{
     self, ParamEnvAnd, ToPolyTraitRef, ToPredicate, Ty, TyCtxt, TypeFoldable, WithConstness,
 };
@@ -1111,9 +1111,16 @@ fn pick_by_value_method(
                 pick.autoderefs = step.autoderefs;

                 // Insert a `&*` or `&mut *` if this is a reference type:
-                if let ty::Ref(_, _, mutbl) = *step.self_ty.value.value.kind() {
-                    pick.autoderefs += 1;
-                    pick.autoref = Some(mutbl);
+                match *step.self_ty.value.value.kind() {
+                    ty::Ref(_, _, mutbl) => {
+                        pick.autoderefs += 1;
+                        pick.autoref = Some(mutbl);
+                    }
+                    ty::RawPtr(TypeAndMut { ty: _, mutbl }) => {
+                        pick.autoderefs += 1;
+                        pick.autoref = Some(mutbl);
+                    }
+                    _ => {}
                 }

                 pick
