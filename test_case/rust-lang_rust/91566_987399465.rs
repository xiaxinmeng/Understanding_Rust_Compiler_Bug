diff
diff --git a/llvm/lib/CodeGen/AsmPrinter/DwarfDebug.cpp b/llvm/lib/CodeGen/AsmPrinter/DwarfDebug.cpp
index c4795fdea914..264cb86eb16a 100644
--- a/llvm/lib/CodeGen/AsmPrinter/DwarfDebug.cpp
+++ b/llvm/lib/CodeGen/AsmPrinter/DwarfDebug.cpp
@@ -1247,10 +1247,12 @@ void DwarfDebug::finalizeModuleInfo() {
                                          ? dwarf::DW_AT_dwo_name
                                          : dwarf::DW_AT_GNU_dwo_name;
       finishUnitAttributes(TheCU.getCUNode(), TheCU);
-      TheCU.addString(TheCU.getUnitDie(), attrDWOName,
-                      Asm->TM.Options.MCOptions.SplitDwarfFile);
-      SkCU->addString(SkCU->getUnitDie(), attrDWOName,
-                      Asm->TM.Options.MCOptions.SplitDwarfFile);
+      StringRef embeddedDWOName = TheCU.getCUNode()->getSplitDebugFilename();
+      if (embeddedDWOName.empty()) {
+        embeddedDWOName = Asm->TM.Options.MCOptions.SplitDwarfFile;
+      }
+      TheCU.addString(TheCU.getUnitDie(), attrDWOName, embeddedDWOName);
+      SkCU->addString(SkCU->getUnitDie(), attrDWOName, embeddedDWOName);
       // Emit a unique identifier for this CU.
       uint64_t ID =
           DIEHash(Asm, &TheCU).computeCUSignature(DWOName, TheCU.getUnitDie());
