 diff
diff --git a/lib/Transforms/IPO/PassManagerBuilder.cpp b/lib/Transforms/IPO/PassManagerBuilder.cpp
index df6a48e..da420f3 100644
--- a/lib/Transforms/IPO/PassManagerBuilder.cpp
+++ b/lib/Transforms/IPO/PassManagerBuilder.cpp
@@ -317,6 +317,9 @@ void PassManagerBuilder::addFunctionSimplificationPasses(
   // Run instcombine after redundancy elimination to exploit opportunities
   // opened up by them.
   addInstructionCombiningPass(MPM);
+  if (OptLevel > 1) {
+    MPM.add(createGVNPass(DisableGVNLoadPRE));  // Remove redundancies
+  }
   addExtensionsToPM(EP_Peephole, MPM);
   MPM.add(createJumpThreadingPass());         // Thread jumps
   MPM.add(createCorrelatedValuePropagationPass());
