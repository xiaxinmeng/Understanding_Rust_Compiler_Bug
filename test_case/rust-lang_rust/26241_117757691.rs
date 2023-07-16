
llc: ../../../../../../../projects/rust/src/llvm/lib/CodeGen/AsmPrinter/DwarfUnit.cpp:525: bool isUnsignedDIType(llvm::DwarfDebug*, const llvm::DIType*): Assertion `T == dwarf::DW_TAG_typedef || T == dwarf::DW_TAG_const_type || T == dwarf::DW_TAG_volatile_type || T == dwarf::DW_TAG_restrict_type || T == dwarf::DW_TAG_enumeration_type' failed.
0  llc             0x0000000001381905 llvm::sys::PrintStackTrace(llvm::raw_ostream&) + 37
1  llc             0x0000000001380471
2  libpthread.so.0 0x00007f01eb9a9660
3  libc.so.6       0x00007f01ea94d528 gsignal + 56
4  libc.so.6       0x00007f01ea94e93a abort + 362
5  libc.so.6       0x00007f01ea9463a7
6  libc.so.6       0x00007f01ea946452
7  llc             0x0000000000cd601b
8  llc             0x0000000000cd61b2 llvm::DwarfUnit::addConstantValue(llvm::DIE&, llvm::MachineOperand const&, llvm::DIType const*) + 34
9  llc             0x0000000000d013d8 llvm::DwarfCompileUnit::constructVariableDIEImpl(llvm::DbgVariable const&, bool) + 504
10 llc             0x0000000000d01784 llvm::DwarfCompileUnit::constructVariableDIE(llvm::DbgVariable&, bool) + 20
11 llc             0x0000000000d017b6 llvm::DwarfCompileUnit::constructVariableDIE(llvm::DbgVariable&, llvm::LexicalScope const&, llvm::DIE*&) + 22
12 llc             0x0000000000d05c6a llvm::DwarfCompileUnit::createScopeChildrenDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&, unsigned int*) + 330
13 llc             0x0000000000d052d6 llvm::DwarfCompileUnit::constructScopeDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&) + 374
14 llc             0x0000000000d05d53 llvm::DwarfCompileUnit::createScopeChildrenDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&, unsigned int*) + 563
15 llc             0x0000000000d056c3 llvm::DwarfCompileUnit::constructScopeDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&) + 1379
16 llc             0x0000000000d05d53 llvm::DwarfCompileUnit::createScopeChildrenDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&, unsigned int*) + 563
17 llc             0x0000000000d052d6 llvm::DwarfCompileUnit::constructScopeDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&) + 374
18 llc             0x0000000000d05d53 llvm::DwarfCompileUnit::createScopeChildrenDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&, unsigned int*) + 563
19 llc             0x0000000000d056c3 llvm::DwarfCompileUnit::constructScopeDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&) + 1379
20 llc             0x0000000000d05d53 llvm::DwarfCompileUnit::createScopeChildrenDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&, unsigned int*) + 563
21 llc             0x0000000000d052d6 llvm::DwarfCompileUnit::constructScopeDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&) + 374
22 llc             0x0000000000d05d53 llvm::DwarfCompileUnit::createScopeChildrenDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&, unsigned int*) + 563
23 llc             0x0000000000d056c3 llvm::DwarfCompileUnit::constructScopeDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&) + 1379
24 llc             0x0000000000d05d53 llvm::DwarfCompileUnit::createScopeChildrenDIE(llvm::LexicalScope*, llvm::SmallVectorImpl<std::unique_ptr<llvm::DIE, std::default_delete<llvm::DIE> > >&, unsigned int*) + 563
25 llc             0x0000000000d05e83 llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) + 67
26 llc             0x0000000000d06188 llvm::DwarfCompileUnit::constructSubprogramScopeDIE(llvm::LexicalScope*) + 280
27 llc             0x0000000000ccdf5f llvm::DwarfDebug::endFunction(llvm::MachineFunction const*) + 1247
28 llc             0x0000000000cb45b8 llvm::AsmPrinter::EmitFunctionBody() + 5160
29 llc             0x00000000005f3136
30 llc             0x0000000001259958 llvm::FPPassManager::runOnFunction(llvm::Function&) + 632
31 llc             0x0000000001259d3b llvm::FPPassManager::runOnModule(llvm::Module&) + 43
32 llc             0x000000000125948f llvm::legacy::PassManagerImpl::run(llvm::Module&) + 783
33 llc             0x00000000005a61d8
34 llc             0x0000000000577a20 main + 352
35 libc.so.6       0x00007f01ea93a790 __libc_start_main + 240
36 llc             0x000000000059fc59 _start + 41
Stack dump:
0.  Program arguments: ./x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/llc std.ll 
1.  Running pass 'Function Pass Manager' on module 'std.ll'.
2.  Running pass 'X86 Assembly / Object Emitter' on function '@"_ZN5boxed16F.FnBox$LT$A$GT$8call_box20h2184742159468874156E"'
zsh: abort (core dumped)  ./x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/llc std.ll
