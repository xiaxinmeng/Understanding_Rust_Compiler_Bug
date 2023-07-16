diff
diff --git a/src/bootstrap/test.rs b/src/bootstrap/test.rs
index 965d1162145..2b6266be7ee 100644
--- a/src/bootstrap/test.rs
+++ b/src/bootstrap/test.rs
@@ -1887,7 +1887,7 @@ fn run(self, builder: &Builder<'_>) {
                 cargo.arg("--doc");
             }
             DocTests::No => {
-                cargo.args(&["--lib", "--bins", "--examples", "--tests", "--benches"]);
+                cargo.args(&["--bins", "--examples", "--tests", "--benches"]);
             }
             DocTests::Yes => {}
         }
