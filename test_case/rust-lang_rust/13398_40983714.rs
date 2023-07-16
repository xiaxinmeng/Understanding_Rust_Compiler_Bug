
diff --git a/src/librustc/middle/ty.rs b/src/librustc/middle/ty.rs
index 5aebd3b..cd33dbf 100644
--- a/src/librustc/middle/ty.rs
+++ b/src/librustc/middle/ty.rs
@@ -1996,6 +1996,10 @@ pub fn type_is_sendable(cx: &ctxt, t: ty::t) -> bool {
     type_contents(cx, t).is_sendable(cx)
 }

+pub fn type_is_sized(cx: &ctxt, t: ty::t) -> bool {
+    type_contents(cx, t).is_sized(cx)
+}
+
 pub fn type_interior_is_unsafe(cx: &ctxt, t: ty::t) -> bool {
     type_contents(cx, t).interior_unsafe()
 }
@@ -2565,35 +2569,6 @@ pub fn type_is_machine(ty: t) -> bool {
     }
 }

-// Is the type's representation size known at compile time?
-#[allow(dead_code)] // leaving in for DST
-pub fn type_is_sized(cx: &ctxt, ty: ty::t) -> bool {
-    match get(ty).sty {
-        ty_param(tp) => {
-            assert_eq!(tp.def_id.krate, ast::LOCAL_CRATE);
-
-            let ty_param_defs = cx.ty_param_defs.borrow();
-            let param_def = ty_param_defs.get(&tp.def_id.node);
-            param_def.bounds.builtin_bounds.contains_elem(BoundSized)
-        },
-        ty_self(def_id) => {
-            let trait_def = lookup_trait_def(cx, def_id);
-            trait_def.bounds.contains_elem(BoundSized)
-        },
-        ty_struct(def_id, ref substs) => {
-            let flds = lookup_struct_fields(cx, def_id);
-            let mut tps = flds.iter().map(|f| lookup_field_type(cx, def_id, f.id, substs));
-            !tps.any(|ty| !type_is_sized(cx, ty))
-        }
-        ty_tup(ref ts) => !ts.iter().any(|t| !type_is_sized(cx, *t)),
-        ty_enum(did, ref substs) => {
-            let variants = substd_enum_variants(cx, did, substs);
-            !variants.iter().any(|v| v.args.iter().any(|t| !type_is_sized(cx, *t)))
-        }
-        _ => true
-    }
-}
-
 // Whether a type is enum like, that is an enum type with only nullary
 // constructors
 pub fn type_is_c_like_enum(cx: &ctxt, ty: t) -> bool {
