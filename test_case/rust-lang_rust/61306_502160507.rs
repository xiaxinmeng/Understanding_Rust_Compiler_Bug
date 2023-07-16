
--- rustc-1.34.2-src/src/rustllvm/RustWrapper.cpp
+++ rustc-1.34.2-src/src/rustllvm/RustWrapper.cpp
@@ -676,8 +677,8 @@
       LineNo, unwrapDI<DISubroutineType>(Ty), ScopeLine, fromRust(Flags),
       fromRust(SPFlags), TParams, unwrapDIPtr<DISubprogram>(Decl));
 #else
-  bool IsLocalToUnit = isSet(SPFlags & LLVMRustDISPFlags::SPFlagLocalToUnit);
-  bool IsDefinition = isSet(SPFlags & LLVMRustDISPFlags::SPFlagDefinition);
+  bool IsLocalToUnit = true; //isSet(SPFlags & LLVMRustDISPFlags::SPFlagLocalToUnit);
+  bool IsDefinition = true; //isSet(SPFlags & LLVMRustDISPFlags::SPFlagDefinition);
   bool IsOptimized = isSet(SPFlags & LLVMRustDISPFlags::SPFlagOptimized);
   DISubprogram *Sub = Builder->createFunction(
       unwrapDI<DIScope>(Scope), Name, LinkageName, unwrapDI<DIFile>(File),
