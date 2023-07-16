
Index: lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp
===================================================================
--- lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp    (revision 137913)
+++ lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp    (working copy)
@@ -4957,11 +4957,16 @@
   }
   case Intrinsic::gcroot:
     if (GFI) {
-      const Value *Alloca = I.getArgOperand(0);
-      const Constant *TypeMap = cast<Constant>(I.getArgOperand(1));
+      const Value *Alloca = I.getArgOperand(0)->stripPointerCasts();
+      assert(isa<AllocaInst>(Alloca) && "First argument to gcroot() must be "
+             "an alloca or a bitcast of one!");

+      const Value *TypeMap = I.getArgOperand(1);
+      assert(isa<Constant>(TypeMap) && "Second argument to gcroot() must be "
+             "a constant!");
+
       FrameIndexSDNode *FI = cast<FrameIndexSDNode>(getValue(Alloca).getNode());
-      GFI->addStackRoot(FI->getIndex(), TypeMap);
+      GFI->addStackRoot(FI->getIndex(), cast<Constant>(TypeMap));
     }
     return 0;
   case Intrinsic::gcread:
