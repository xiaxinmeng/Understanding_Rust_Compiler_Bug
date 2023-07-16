
#0  llvm::SDNode::SDNode (this=0x55cae6cb3218, Opc=271, Order=7, dl=..., VTs=...)
    at ../include/llvm/CodeGen/SelectionDAGNodes.h:989
#1  0x000055cae31761a5 in llvm::SelectionDAG::newSDNode<llvm::SDNode, unsigned int&, unsigned int, llvm::DebugLoc const&, llvm::SDVTList> (this=0x55cae62a9a00)
    at ../include/llvm/CodeGen/SelectionDAG.h:319
#2  0x000055cae3185b99 in llvm::SelectionDAG::getNode (this=this@entry=0x55cae62a9a00, 
    Opcode=Opcode@entry=271, DL=..., VT=..., N1=..., N2=..., Flags=..., Flags@entry=...)
    at ../lib/CodeGen/SelectionDAG/SelectionDAG.cpp:4726
#3  0x000055cae26b9c37 in llvm::X86TargetLowering::EmitTest (this=this@entry=0x55cae6ba8d68, 
    Op=..., X86CC=X86CC@entry=4, dl=..., DAG=...) at ../lib/Target/X86/X86ISelLowering.cpp:17294
#4  0x000055cae26bbccc in llvm::X86TargetLowering::EmitCmp (this=this@entry=0x55cae6ba8d68, 
    Op0=..., Op1=..., X86CC=X86CC@entry=4, dl=..., DAG=...)
    at ../lib/Target/X86/X86ISelLowering.cpp:17309
#5  0x000055cae26bc0dc in llvm::X86TargetLowering::LowerSETCC (this=this@entry=0x55cae6ba8d68, 
    Op=..., DAG=...) at ../lib/Target/X86/X86ISelLowering.cpp:18148
#6  0x000055cae26bec4a in llvm::X86TargetLowering::LowerBRCOND (this=0x55cae6ba8d68, Op=..., 
    DAG=...) at ../lib/Target/X86/X86ISelLowering.cpp:19161
#7  0x000055cae26d038b in llvm::X86TargetLowering::LowerOperation (this=<optimized out>, Op=..., 
    DAG=...) at ../lib/Target/X86/X86ISelLowering.cpp:24746
#8  0x000055cae30efcf3 in (anonymous namespace)::SelectionDAGLegalize::LegalizeOp (
    this=0x7fff8e577cb0, Node=0x55cae6ca6de0) at ../lib/CodeGen/SelectionDAG/LegalizeDAG.cpp:1211
#9  0x000055cae30f4051 in llvm::SelectionDAG::Legalize (this=0x55cae62a9a00)
    at ../lib/CodeGen/SelectionDAG/LegalizeDAG.cpp:4682
#10 0x000055cae31a581a in llvm::SelectionDAGISel::CodeGenAndEmitDAG (
    this=this@entry=0x55cae60209a0) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:834
#11 0x000055cae31a685a in llvm::SelectionDAGISel::SelectBasicBlock (
    this=this@entry=0x55cae60209a0, Begin=..., Begin@entry=..., End=..., End@entry=..., 
    HadTailCall=@0x7fff8e5785a0: false) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:664
#12 0x000055cae31b040f in llvm::SelectionDAGISel::SelectAllBasicBlocks (
    this=this@entry=0x55cae60209a0, Fn=...)
    at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:1624
#13 0x000055cae31b22f2 in llvm::SelectionDAGISel::runOnMachineFunction (this=<optimized out>, 
    mf=...) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:466
#14 0x000055cae31b3aac in llvm::SelectionDAGISel::runOnMachineFunction (this=<optimized out>, 
    mf=...) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:628
#15 0x000055cae2604c74 in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction (
    this=<optimized out>, MF=...) at ../lib/Target/X86/X86ISelDAGToDAG.cpp:175
#16 0x000055cae2a58b35 in llvm::MachineFunctionPass::runOnFunction (this=0x55cae60209a0, F=...)
    at ../lib/CodeGen/MachineFunctionPass.cpp:62
#17 0x000055cae2d76fd0 in llvm::FPPassManager::runOnFunction (this=0x55cae6a59f10, F=...)
    at ../lib/IR/LegacyPassManager.cpp:1520
#18 0x000055cae2d77039 in llvm::FPPassManager::runOnModule (this=0x55cae6a59f10, M=...)
    at ../lib/IR/LegacyPassManager.cpp:1541
#19 0x000055cae2d767d1 in (anonymous namespace)::MPPassManager::runOnModule (M=..., 
    this=<optimized out>) at ../lib/IR/LegacyPassManager.cpp:1597
#20 llvm::legacy::PassManagerImpl::run (this=0x55cae62a58c0, M=...)
    at ../lib/IR/LegacyPassManager.cpp:1700
#21 0x000055cae258b7d0 in compileModule (argv=<optimized out>, Context=...)
    at ../tools/llc/llc.cpp:569
#22 0x000055cae254c85c in main (argc=<optimized out>, argv=0x7fff8e579368)
    at ../tools/llc/llc.cpp:346
