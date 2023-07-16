
#0  llvm::SDNode::SDNode (this=this@entry=0x55bdbcfa5c30, Opc=Opc@entry=186, Order=4, 
    dl=dl@entry=..., VTs=...) at ../include/llvm/CodeGen/SelectionDAGNodes.h:989
#1  0x000055bdb9717d6f in llvm::MemSDNode::MemSDNode (this=this@entry=0x55bdbcfa5c30, 
    Opc=Opc@entry=186, Order=<optimized out>, dl=..., VTs=..., memvt=..., mmo=0x55bdbd086bb0)
    at ../lib/CodeGen/SelectionDAG/SelectionDAG.cpp:7683
#2  0x000055bdb9721b6f in llvm::LSBaseSDNode::LSBaseSDNode (MMO=<optimized out>, MemVT=..., AM=8, 
    VTs=..., dl=..., Order=<optimized out>, NodeTy=llvm::ISD::LOAD, this=0x55bdbcfa5c30)
    at ../include/llvm/CodeGen/SelectionDAGNodes.h:1945
#3  llvm::LoadSDNode::LoadSDNode (MMO=0x55bdbd086bb0, MemVT=..., ETy=<optimized out>, AM=8, 
    VTs=..., dl=..., Order=<optimized out>, this=0x55bdbcfa5c30)
    at ../include/llvm/CodeGen/SelectionDAGNodes.h:1979
#4  llvm::SelectionDAG::newSDNode<llvm::LoadSDNode, unsigned int, llvm::DebugLoc const&, llvm::SDVTList&, llvm::ISD::MemIndexedMode&, llvm::ISD::LoadExtType&, llvm::EVT&, llvm::MachineMemOperand*&> (
    this=0x55bdbc074400) at ../include/llvm/CodeGen/SelectionDAG.h:319
#5  llvm::SelectionDAG::getLoad (this=this@entry=0x55bdbc074400, AM=AM@entry=llvm::ISD::UNINDEXED, 
    ExtType=<optimized out>, ExtType@entry=llvm::ISD::NON_EXTLOAD, VT=..., dl=..., Chain=..., 
    Ptr=..., Offset=..., MemVT=..., MMO=0x55bdbd086bb0)
    at ../lib/CodeGen/SelectionDAG/SelectionDAG.cpp:5941
#6  0x000055bdb9721ff3 in llvm::SelectionDAG::getLoad (this=this@entry=0x55bdbc074400, 
    AM=AM@entry=llvm::ISD::UNINDEXED, ExtType=ExtType@entry=llvm::ISD::NON_EXTLOAD, VT=..., 
    dl=..., Chain=..., Ptr=..., Offset=..., PtrInfo=..., MemVT=..., Alignment=8, MMOFlags=17, 
    AAInfo=..., Ranges=0x0) at ../lib/CodeGen/SelectionDAG/SelectionDAG.cpp:5899
#7  0x000055bdb9728752 in llvm::SelectionDAG::getLoad (this=0x55bdbc074400, VT=..., dl=..., 
    Chain=..., Ptr=..., PtrInfo=..., Alignment=8, MMOFlags=17, AAInfo=..., Ranges=0x0)
    at ../lib/CodeGen/SelectionDAG/SelectionDAG.cpp:5958
#8  0x000055bdb96616e1 in (anonymous namespace)::DAGCombiner::ReplaceExtractVectorEltOfLoadWithNarrowedLoad (this=this@entry=0x7ffe58777920, EVE=EVE@entry=0x55bdbccf0368, InVecVT=..., EltNo=..., 
    OriginalLoad=0x55bdbccf0230) at ../lib/CodeGen/SelectionDAG/DAGCombiner.cpp:14172
#9  0x000055bdb966221e in (anonymous namespace)::DAGCombiner::visitEXTRACT_VECTOR_ELT (
    this=0x7ffe58777920, N=0x55bdbccf0368) at ../lib/CodeGen/SelectionDAG/DAGCombiner.cpp:14405
#10 0x000055bdb966f444 in (anonymous namespace)::DAGCombiner::visit (
    this=this@entry=0x7ffe58777920, N=N@entry=0x55bdbccf0368)
    at ../lib/CodeGen/SelectionDAG/DAGCombiner.cpp:1601
#11 0x000055bdb9670e8c in (anonymous namespace)::DAGCombiner::combine (
    this=this@entry=0x7ffe58777920, N=N@entry=0x55bdbccf0368)
    at ../lib/CodeGen/SelectionDAG/DAGCombiner.cpp:1619
#12 0x000055bdb96727b7 in (anonymous namespace)::DAGCombiner::Run (this=this@entry=0x7ffe58777920, 
    AtLevel=AtLevel@entry=llvm::AfterLegalizeDAG)
    at ../lib/CodeGen/SelectionDAG/DAGCombiner.cpp:1466
#13 0x000055bdb9674d65 in llvm::SelectionDAG::Combine (this=<optimized out>, 
    Level=Level@entry=llvm::AfterLegalizeDAG, AA=<optimized out>, OptLevel=<optimized out>)
    at ../lib/CodeGen/SelectionDAG/DAGCombiner.cpp:17752
#14 0x000055bdb974f89f in llvm::SelectionDAGISel::CodeGenAndEmitDAG (
    this=this@entry=0x55bdbc0599a0) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:849
#15 0x000055bdb975085a in llvm::SelectionDAGISel::SelectBasicBlock (
    this=this@entry=0x55bdbc0599a0, Begin=..., Begin@entry=..., End=..., End@entry=..., 
    HadTailCall=@0x7ffe58778460: false) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:664
#16 0x000055bdb975a40f in llvm::SelectionDAGISel::SelectAllBasicBlocks (
    this=this@entry=0x55bdbc0599a0, Fn=...)
    at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:1624
#17 0x000055bdb975c2f2 in llvm::SelectionDAGISel::runOnMachineFunction (this=<optimized out>, 
    mf=...) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:466
#18 0x000055bdb975daac in llvm::SelectionDAGISel::runOnMachineFunction (this=<optimized out>, 
    mf=...) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:628
#19 0x000055bdb8baec74 in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction (
    this=<optimized out>, MF=...) at ../lib/Target/X86/X86ISelDAGToDAG.cpp:175
#20 0x000055bdb9002b35 in llvm::MachineFunctionPass::runOnFunction (this=0x55bdbc0599a0, F=...)
    at ../lib/CodeGen/MachineFunctionPass.cpp:62
#21 0x000055bdb9320fd0 in llvm::FPPassManager::runOnFunction (this=0x55bdbcaaa330, F=...)
    at ../lib/IR/LegacyPassManager.cpp:1520
#22 0x000055bdb9321039 in llvm::FPPassManager::runOnModule (this=0x55bdbcaaa330, M=...)
    at ../lib/IR/LegacyPassManager.cpp:1541
#23 0x000055bdb93207d1 in (anonymous namespace)::MPPassManager::runOnModule (M=..., 
    this=<optimized out>) at ../lib/IR/LegacyPassManager.cpp:1597
#24 llvm::legacy::PassManagerImpl::run (this=0x55bdbc258f30, M=...)
    at ../lib/IR/LegacyPassManager.cpp:1700
#25 0x000055bdb8b357d0 in compileModule (argv=<optimized out>, Context=...)
    at ../tools/llc/llc.cpp:569
#26 0x000055bdb8af685c in main (argc=<optimized out>, argv=0x7ffe58779228)
    at ../tools/llc/llc.cpp:346
