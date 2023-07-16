diff
diff --git a/src/test/run-pass/rustc-rust-log.rs b/src/test/run-pass/rustc-rust-log.rs
index 629387d4cb..a0e04beaa9 100644
--- a/src/test/run-pass/rustc-rust-log.rs
+++ b/src/test/run-pass/rustc-rust-log.rs
@@ -9,5 +9,6 @@
 // except according to those terms.
 
 // rustc-env:RUST_LOG=debug
+// ignore-musl - workaround to avoid consuming all CI storage, see #48118
