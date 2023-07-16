 diff
diff --git a/src/librustc/middle/resolve.rs b/src/librustc/middle/resolve.rs
index 7eb9b23..99ce3f9 100644
--- a/src/librustc/middle/resolve.rs
+++ b/src/librustc/middle/resolve.rs
@@ -5177,7 +5177,7 @@ impl Resolver {
                         let rib = label_ribs.get()[label_ribs.get().len() -
                                                    1];
                         let mut bindings = rib.bindings.borrow_mut();
-                        bindings.get().insert(label.name, def_like);
+                        bindings.get().insert(mtwt_resolve(label), def_like);
                     }

                     visit::walk_expr(this, expr, ());
