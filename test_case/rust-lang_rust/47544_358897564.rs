
diff --git a/src/tools/compiletest/src/runtest.rs b/src/tools/compiletest/src/runtest.rs
index dbeee39e60..175a829807 100644
--- a/src/tools/compiletest/src/runtest.rs
+++ b/src/tools/compiletest/src/runtest.rs
@@ -212,11 +212,9 @@ impl<'test> TestCx<'test> {
     }
 
     fn check_correct_failure_status(&self, proc_res: &ProcRes) {
-        // The value the rust runtime returns on failure
-        const RUST_ERR: i32 = 101;
-        if proc_res.status.code() != Some(RUST_ERR) {
+        if proc_res.status.success() {
             self.fatal_proc_rec(
-                &format!("failure produced the wrong error: {}", proc_res.status),
+                "failure must not return success exit status!",
                 proc_res,
             );
         }
