diff
-     #[cfg(target_os = "nto")]
+    #[cfg(all(target_os = "nto", target_env = "nto710"))]
