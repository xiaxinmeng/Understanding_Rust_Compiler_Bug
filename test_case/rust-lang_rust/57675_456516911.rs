diff
diff --git a/llvm/lib/CodeGen/AsmPrinter/CodeViewDebug.cpp b/llvm/lib/CodeGen/AsmPrinter/CodeViewDebug.cpp
index 75ecd81f2101..60731f1e676c 100644
--- a/llvm/lib/CodeGen/AsmPrinter/CodeViewDebug.cpp
+++ b/llvm/lib/CodeGen/AsmPrinter/CodeViewDebug.cpp
@@ -1835,7 +1835,10 @@ TypeIndex CodeViewDebug::lowerTypeMemberFunction(const DISubroutineType *Ty,

   unsigned Index = 0;
   SmallVector<TypeIndex, 8> ArgTypeIndices;
-  TypeIndex ReturnTypeIndex = getTypeIndex(ReturnAndArgs[Index++]);
+  TypeIndex ReturnTypeIndex = TypeIndex::Void();
+  if (ReturnAndArgs.size() > Index) {
+    ReturnTypeIndex = getTypeIndex(ReturnAndArgs[Index++]);
+  }

   // If the first argument is a pointer type and this isn't a static method,
   // treat it as the special 'this' parameter, which is encoded separately from
