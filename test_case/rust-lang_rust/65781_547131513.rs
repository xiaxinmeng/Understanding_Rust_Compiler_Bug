diff
@@ -243,17 +245,32 @@ provide! { <'tcx> tcx, def_id, other, cdata,
         // from this crate.
         let formats = tcx.dependency_formats(LOCAL_CRATE);
         let remove_generics = formats.iter().any(|(_ty, list)| {
             match list.get(def_id.krate.as_usize() - 1) {
-                Some(Linkage::IncludedFromDylib) | Some(Linkage::Dynamic) => true,
-                _ => false,
+                Some(Linkage::Static) => false,
+                _ => true,
              }
         });
