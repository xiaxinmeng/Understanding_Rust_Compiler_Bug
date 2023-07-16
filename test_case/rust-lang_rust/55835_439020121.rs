diff
diff --git a/src/rustllvm/PassWrapper.cpp b/src/rustllvm/PassWrapper.cpp
index 06f75d981e..04efbebbda 100644
--- a/src/rustllvm/PassWrapper.cpp
+++ b/src/rustllvm/PassWrapper.cpp
@@ -1004,7 +1004,7 @@ LLVMRustCreateThinLTOData(LLVMRustThinLTOModule *modules,
                               GlobalValue::LinkageTypes NewLinkage) {
     ResolvedODR[ModuleIdentifier][GUID] = NewLinkage;
   };
-  thinLTOResolveWeakForLinkerInIndex(Ret->Index, isPrevailing, recordNewLinkage);
+  thinLTOResolvePrevailingInIndex(Ret->Index, isPrevailing, recordNewLinkage);

   // Here we calculate an `ExportedGUIDs` set for use in the `isExported`
   // callback below. This callback below will dictate the linkage for all
@@ -1067,7 +1067,7 @@ extern "C" bool
 LLVMRustPrepareThinLTOResolveWeak(const LLVMRustThinLTOData *Data, LLVMModuleRef M) {
   Module &Mod = *unwrap(M);
   const auto &DefinedGlobals = Data->ModuleToDefinedGVSummaries.lookup(Mod.getModuleIdentifier());
-  thinLTOResolveWeakForLinkerModule(Mod, DefinedGlobals);
+  thinLTOResolvePrevailingInModule(Mod, DefinedGlobals);
   return true;
 }
