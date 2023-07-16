diff
diff --git a/lib/IR/Globals.cpp b/lib/IR/Globals.cpp
index da1b6c5e0c9..a742c4af4fd 100644
--- a/lib/IR/Globals.cpp
+++ b/lib/IR/Globals.cpp
@@ -250,6 +250,19 @@ bool GlobalValue::canIncreaseAlignment() const {
   if (isELF && hasDefaultVisibility() && !hasLocalLinkage())
     return false;
 
+  // The dynamic linker on macOS 10.10 has a bug (radar 24221680):
+  // it does not respect the alignment given on the TLS section.
+  // It is thus unsafe to increase the alignment since the runtime
+  // would violate the assumption and may cause segmentation fault.
+  //
+  // See also:
+  //  - <https://github.com/ldc-developers/ldc/issues/1252>
+  //  - <https://github.com/rust-lang/rust/issues/44056>
+  bool isMachO =
+      (!Parent || Triple(Parent->getTargetTriple()).isOSBinFormatMachO());
+  if (isMachO && isThreadLocal())
+    return false;
+
   return true;
 }
