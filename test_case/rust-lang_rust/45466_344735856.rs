
diff --git a/lib/Transforms/IPO/PassManagerBuilder.cpp b/lib/Transforms/IPO/PassManagerBuilder.cpp
index 260b2283b31..baa38d94d17 100644
--- a/lib/Transforms/IPO/PassManagerBuilder.cpp
+++ b/lib/Transforms/IPO/PassManagerBuilder.cpp
@@ -1,3 +1,4 @@
+
 //===- PassManagerBuilder.cpp - Build Standard Pass -----------------------===//
 //
 //                     The LLVM Compiler Infrastructure
@@ -315,6 +316,7 @@ void PassManagerBuilder::addFunctionSimplificationPasses(
   MPM.add(createCFGSimplificationPass());
   addInstructionCombiningPass(MPM);
   MPM.add(createIndVarSimplifyPass());        // Canonicalize indvars
+  MPM.add(createCFGSimplificationPass());     // Clean up after IndVarSimplify
   MPM.add(createLoopIdiomPass());             // Recognize idioms like memset.
   MPM.add(createLoopDeletionPass());          // Delete dead loops
   if (EnableLoopInterchange) {
