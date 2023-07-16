
-            .self_desc
-            .clone()
-            .clone()
-            .map_or_else(|| String::new(), |ty| format!(" for type `{}`", ty))
+        overlap.self_desc.clone().map_or_else(|| String::new(), |ty| format!(" for type `{}`", ty))
