patch
diff --git a/compiler/rustc_target/src/spec/x86_64_unknown_linux_gnu.rs b/compiler/rustc_target/src/spec/x86_64_unknown_linux_gnu.rs
index 956be0353fa..51eec0a3a10 100644
--- a/compiler/rustc_target/src/spec/x86_64_unknown_linux_gnu.rs
+++ b/compiler/rustc_target/src/spec/x86_64_unknown_linux_gnu.rs
@@ -13,6 +13,7 @@ pub fn target() -> Target {
         | SanitizerSet::LEAK
         | SanitizerSet::MEMORY
         | SanitizerSet::THREAD;
+    base.has_thread_local = false;
 
     Target {
         llvm_target: "x86_64-unknown-linux-gnu".into(),
