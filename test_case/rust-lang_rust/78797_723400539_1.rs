diff
                 }
-            } else if path_str.starts_with("crate::") {
+            } else if path_str.starts_with("crate::") || path_str == "crate" {
                 // Resolve `crate` relative to the original scope of the item, not the current scope.
