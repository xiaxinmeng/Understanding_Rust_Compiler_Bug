
         if done_stamp.exists() {
             if builder.config.llvm_skip_rebuild {
-                builder.info("Warning: \
+                builder.info(
+                    "Warning: \
                               Using a potentially stale build of LLVM; \
-                              This may not behave well.");
+                              This may not behave well.",
+                );
                 return build_llvm_config;
             }
