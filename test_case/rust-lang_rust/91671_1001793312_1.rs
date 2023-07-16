diff
diff --git a/llvm/lib/CodeGen/GlobalISel/CombinerHelper.cpp b/llvm/lib/CodeGen/GlobalISel/CombinerHelper.cpp
index 06d827de2e96..aef9ba02b000 100644
--- a/llvm/lib/CodeGen/GlobalISel/CombinerHelper.cpp
+++ b/llvm/lib/CodeGen/GlobalISel/CombinerHelper.cpp
@@ -161,7 +161,9 @@ void CombinerHelper::applyCombineCopy(MachineInstr &MI) {
   Register DstReg = MI.getOperand(0).getReg();
   Register SrcReg = MI.getOperand(1).getReg();
   MI.eraseFromParent();
-  replaceRegWith(MRI, DstReg, SrcReg);
+  if (DstReg != SrcReg) {
+      replaceRegWith(MRI, DstReg, SrcReg);
+  }
 }

 bool CombinerHelper::tryCombineConcatVectors(MachineInstr &MI) {
