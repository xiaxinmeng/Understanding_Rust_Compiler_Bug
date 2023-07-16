diff
diff --git a/src/librustdoc/clean/inline.rs b/src/librustdoc/clean/inline.rs
index cb70f465f62..f75fa3c2202 100644
--- a/src/librustdoc/clean/inline.rs
+++ b/src/librustdoc/clean/inline.rs
@@ -474,10 +474,6 @@ fn merge_attrs(
         }
     }
 
-    if let Some(did) = trait_.as_ref().map(|t| t.def_id()) {
-        record_extern_trait(cx, did);
-    }
-
     let (merged_attrs, cfg) = merge_attrs(cx, parent_module.into(), load_attrs(cx, did), attrs);
     trace!("merged_attrs={:?}", merged_attrs);
 
diff --git a/src/librustdoc/clean/utils.rs b/src/librustdoc/clean/utils.rs
index 2fae3163a1a..ef12c5c1f1a 100644
--- a/src/librustdoc/clean/utils.rs
+++ b/src/librustdoc/clean/utils.rs
@@ -439,9 +439,6 @@ fn print_const_with_custom_print_scalar(tcx: TyCtxt<'_>, ct: &'tcx ty::Const<'tc
         return did;
     }
     inline::record_extern_fqn(cx, did, kind);
-    if let ItemType::Trait = kind {
-        inline::record_extern_trait(cx, did);
-    }
     did
 }
 
