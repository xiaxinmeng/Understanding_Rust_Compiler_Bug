diff
diff --git a/src/test/compile-fail/allocator-dylib-is-system.rs b/src/test/compile-fail/allocator-dylib-is-system.rs
index 4c576de..1a48ccd 100644
--- a/src/test/compile-fail/allocator-dylib-is-system.rs
+++ b/src/test/compile-fail/allocator-dylib-is-system.rs
@@ -12,7 +12,7 @@
 // aux-build:allocator-dylib.rs
 // aux-build:allocator1.rs
 // no-prefer-dynamic
-// error-pattern: cannot link together two allocators
+// error-pattern: cannot link together two allocators|can't find crate for `alloc_jemalloc`

 // Verify that the allocator for statically linked dynamic libraries is the
 // system allocator. Do this by linking in jemalloc and making sure that we get
diff --git a/src/tools/compiletest/src/runtest.rs b/src/tools/compiletest/src/runtest.rs
index 1ec0838..e7e8f53 100644
--- a/src/tools/compiletest/src/runtest.rs
+++ b/src/tools/compiletest/src/runtest.rs
@@ -987,20 +987,23 @@ actual:\n\
         }
         let mut next_err_idx = 0;
         let mut next_err_pat = self.props.error_patterns[next_err_idx].trim();
-        let mut done = false;
+        let mut next_or_pats: Vec<&str> = next_err_pat.split('|').collect();
+
         for line in output_to_check.lines() {
-            if line.contains(next_err_pat) {
-                debug!("found error pattern {}", next_err_pat);
-                next_err_idx += 1;
-                if next_err_idx == self.props.error_patterns.len() {
-                    debug!("found all error patterns");
-                    done = true;
+            for pat in next_or_pats.clone().iter() {
+                if line.contains(pat) {
+                    debug!("found error pattern {}", next_err_pat);
+                    next_err_idx += 1;
+                    if next_err_idx == self.props.error_patterns.len() {
+                        debug!("found all error patterns");
+                        return;
+                    }
+                    next_err_pat = self.props.error_patterns[next_err_idx].trim();
+                    next_or_pats = next_err_pat.split('|').collect();
                     break;
                 }
-                next_err_pat = self.props.error_patterns[next_err_idx].trim();
             }
         }
-        if done { return; }

         let missing_patterns = &self.props.error_patterns[next_err_idx..];
         if missing_patterns.len() == 1 {
