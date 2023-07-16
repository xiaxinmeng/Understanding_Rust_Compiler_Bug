
diff --git a/src/rustllvm/RustWrapper.cpp b/src/rustllvm/RustWrapper.cpp
index 3dbde46f76..2a5bece1fe 100644
--- a/src/rustllvm/RustWrapper.cpp
+++ b/src/rustllvm/RustWrapper.cpp
@@ -796,7 +796,7 @@ extern "C" LLVMMetadataRef LLVMRustDIBuilderCreateStaticVariable(
   llvm::DIGlobalVariableExpression *VarExpr = Builder->createGlobalVariableExpression(
       unwrapDI<DIDescriptor>(Context), Name, LinkageName,
       unwrapDI<DIFile>(File), LineNo, unwrapDI<DIType>(Ty), IsLocalToUnit,
-      InitExpr, unwrapDIPtr<MDNode>(Decl), AlignInBits);
+      InitExpr, unwrapDIPtr<MDNode>(Decl), nullptr, AlignInBits);

   InitVal->setMetadata("dbg", VarExpr);
