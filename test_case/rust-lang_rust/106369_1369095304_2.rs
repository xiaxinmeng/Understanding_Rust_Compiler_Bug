patch
diff --git a/compiler/rustc_codegen_llvm/src/abi.rs b/compiler/rustc_codegen_llvm/src/abi.rs
index a6fd2a7de6b..30006ac5e7b 100644
--- a/compiler/rustc_codegen_llvm/src/abi.rs
+++ b/compiler/rustc_codegen_llvm/src/abi.rs
@@ -81,7 +81,6 @@ fn should_use_mutable_noalias(cx: &CodegenCx<'_, '_>) -> bool {
             } else {
                 attrs.push(llvm::CreateDereferenceableOrNullAttr(cx.llcx, deref));
             }
-            regular -= ArgAttribute::NonNull;
         }
         for (attr, llattr) in OPTIMIZATION_ATTRIBUTES {
             if regular.contains(attr) {
