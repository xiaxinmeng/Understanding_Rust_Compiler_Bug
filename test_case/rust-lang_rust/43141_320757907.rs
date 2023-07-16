Rust

+    let relevant: Vec<DefId> = all_trait_impls
+        .non_blanket_impls
+        .iter()
+        .cloned()
+        .filter(|&impl_def_id| {
+            let impl_self_ty = tcx.type_of(impl_def_id);
+            let impl_simple_self_ty = fast_reject::simplify_type(tcx,
+                                                                 impl_self_ty,
+                                                                 false).unwrap();
+            impl_simple_self_ty == self_ty
+        })
+        .collect(); 
