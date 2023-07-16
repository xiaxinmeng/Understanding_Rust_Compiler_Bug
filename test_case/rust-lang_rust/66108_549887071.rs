diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 2748903f2d4..de4e6b56459 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -886,6 +886,7 @@ impl<'a> Builder<'a> {
         // things still build right, please do!
         match mode {
             Mode::Std => metadata.push_str("std"),
+            Mode::ToolRustc => metadata.push_str("tool-rustc"),
             _ => {},
         }
         cargo.env("__CARGO_DEFAULT_LIB_METADATA", &metadata);
