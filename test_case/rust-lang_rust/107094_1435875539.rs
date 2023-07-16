
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index ca5f500f93b..6788151f806 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -286,6 +286,9 @@ pub fn prepare_tool_cargo(
     if let Some(date) = info.commit_date() {
         cargo.env("CFG_COMMIT_DATE", date);
     }
+    if let Some(ver_date) = info.commit_date() {
+        cargo.env("CFG_VER_DATE", ver_date);
+    }
     if !features.is_empty() {
         cargo.arg("--features").arg(&features.join(", "));
     }
