diff
diff --git a/llvm/lib/Target/RISCV/RISCVInstrInfo.cpp b/llvm/lib/Target/RISCV/RISCVInstrInfo.cpp
index f6bc52968a3..32c09674ed4 100644
--- a/llvm/lib/Target/RISCV/RISCVInstrInfo.cpp
+++ b/llvm/lib/Target/RISCV/RISCVInstrInfo.cpp
@@ -386,9 +386,6 @@ unsigned RISCVInstrInfo::insertIndirectBranch(MachineBasicBlock &MBB,
   MachineRegisterInfo &MRI = MF->getRegInfo();
   const auto &TM = static_cast<const RISCVTargetMachine &>(MF->getTarget());
 
-  if (TM.isPositionIndependent())
-    report_fatal_error("Unable to insert indirect branch");
-
   if (!isInt<32>(BrOffset))
     report_fatal_error(
         "Branch offsets outside of the signed 32-bit range not supported");
@@ -398,16 +395,29 @@ unsigned RISCVInstrInfo::insertIndirectBranch(MachineBasicBlock &MBB,
   // uses the same workaround).
   Register ScratchReg = MRI.createVirtualRegister(&RISCV::GPRRegClass);
   auto II = MBB.end();
+  unsigned Scav;
+
+  if (TM.isPositionIndependent()) {
+    MBB.setLabelMustBeEmitted();
+    MachineInstr &AuipcMI = *BuildMI(MBB, II, DL, get(RISCV::AUIPC), ScratchReg)
+                                 .addMBB(&DestBB, RISCVII::MO_PCREL_HI);
+    BuildMI(MBB, II, DL, get(RISCV::PseudoBRIND))
+        .addReg(ScratchReg, RegState::Kill)
+        .addMBB(&MBB, RISCVII::MO_PCREL_LO);
+    RS->enterBasicBlockEnd(MBB);
+    Scav = RS->scavengeRegisterBackwards(RISCV::GPRRegClass,
+                                         AuipcMI.getIterator(), false, 0);
+  } else {
+    MachineInstr &LuiMI = *BuildMI(MBB, II, DL, get(RISCV::LUI), ScratchReg)
+                               .addMBB(&DestBB, RISCVII::MO_HI);
+    BuildMI(MBB, II, DL, get(RISCV::PseudoBRIND))
+        .addReg(ScratchReg, RegState::Kill)
+        .addMBB(&DestBB, RISCVII::MO_LO);
+    RS->enterBasicBlockEnd(MBB);
+    Scav = RS->scavengeRegisterBackwards(RISCV::GPRRegClass,
+                                         LuiMI.getIterator(), false, 0);
+  }
 
-  MachineInstr &LuiMI = *BuildMI(MBB, II, DL, get(RISCV::LUI), ScratchReg)
-                             .addMBB(&DestBB, RISCVII::MO_HI);
-  BuildMI(MBB, II, DL, get(RISCV::PseudoBRIND))
-      .addReg(ScratchReg, RegState::Kill)
-      .addMBB(&DestBB, RISCVII::MO_LO);
-
-  RS->enterBasicBlockEnd(MBB);
-  unsigned Scav = RS->scavengeRegisterBackwards(RISCV::GPRRegClass,
-                                                LuiMI.getIterator(), false, 0);
   MRI.replaceRegWith(ScratchReg, Scav);
   MRI.clearVirtRegs();
   RS->setRegUsed(Scav);
diff --git a/llvm/lib/Target/RISCV/RISCVTargetMachine.cpp b/llvm/lib/Target/RISCV/RISCVTargetMachine.cpp
index de71c01753d..9b1bd7f28ec 100644
--- a/llvm/lib/Target/RISCV/RISCVTargetMachine.cpp
+++ b/llvm/lib/Target/RISCV/RISCVTargetMachine.cpp
@@ -32,6 +32,11 @@
 #include "llvm/Target/TargetOptions.h"
 using namespace llvm;
 
+static cl::opt<bool>
+    BranchRelaxation("riscv-enable-branch-relax", cl::Hidden, cl::init(true),
+                     cl::desc("Relax out of range conditional branches"));
+
+
 extern "C" LLVM_EXTERNAL_VISIBILITY void LLVMInitializeRISCVTarget() {
   RegisterTargetMachine<RISCVTargetMachine> X(getTheRISCV32Target());
   RegisterTargetMachine<RISCVTargetMachine> Y(getTheRISCV64Target());
@@ -126,7 +131,6 @@ public:
   bool addLegalizeMachineIR() override;
   bool addRegBankSelect() override;
   bool addGlobalInstructionSelect() override;
-  void addPreEmitPass() override;
   void addPreEmitPass2() override;
   void addPreRegAlloc() override;
 };
@@ -167,13 +171,14 @@ bool RISCVPassConfig::addGlobalInstructionSelect() {
   return false;
 }
 
-void RISCVPassConfig::addPreEmitPass() { addPass(&BranchRelaxationPassID); }
-
 void RISCVPassConfig::addPreEmitPass2() {
   // Schedule the expansion of AMOs at the last possible moment, avoiding the
   // possibility for other passes to break the requirements for forward
   // progress in the LR/SC block.
   addPass(createRISCVExpandPseudoPass());
+
+  if (BranchRelaxation)
+    addPass(&BranchRelaxationPassID);
 }
 
 void RISCVPassConfig::addPreRegAlloc() {
