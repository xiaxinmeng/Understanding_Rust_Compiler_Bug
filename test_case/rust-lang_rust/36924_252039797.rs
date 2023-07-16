 diff
diff --git a/lib/Transforms/Utils/SimplifyCFG.cpp b/lib/Transforms/Utils/SimplifyCFG.cpp
index 90ce672..2702b76 100644
--- a/lib/Transforms/Utils/SimplifyCFG.cpp
+++ b/lib/Transforms/Utils/SimplifyCFG.cpp
@@ -4503,7 +4503,8 @@ GetCaseResults(SwitchInst *SI, ConstantInt *CaseVal, BasicBlock *CaseDest,
        ++I) {
     if (TerminatorInst *T = dyn_cast<TerminatorInst>(I)) {
       // If the terminator is a simple branch, continue to the next block.
-      if (T->getNumSuccessors() != 1)
+      const auto *BI = dyn_cast<BranchInst>(T);
+      if (!BI || !BI->isUnconditional())
         return false;
       Pred = CaseDest;
       CaseDest = T->getSuccessor(0);
