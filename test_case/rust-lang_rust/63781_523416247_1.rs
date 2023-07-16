diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 4e49aaa16ea..3a5901156b4 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -875,7 +875,8 @@ impl<'a> Builder<'a> {
         }
 
         if cmd == "clippy" {
-            extra_args.push_str("-Zforce-unstable-if-unmarked");
+            extra_args.push_str("-Zforce-unstable-if-unmarked \
+            --json=diagnostic-rendered-ansi --error-format=json");
         }
 
         if !extra_args.is_empty() {
