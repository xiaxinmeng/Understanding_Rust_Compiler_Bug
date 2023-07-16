
[mips][mc] Fix a crash when disassembling odd sized sections
Use correct registers for "A" inline asm constraint
[X86] Remove special handling for 16 bit for A asm constraints.
Fix invalid addrspacecast due to combining alloca with global var
Adding const overloads of operator* and operator-> for DenseSet iterators
[Hexagon] Fix lowering of formal arguments of type i1
[InstCombine] Fix bug in pointer replacement
[CMake] Fix pthread handling for out-of-tree builds
CMake: Don't install llvm-tblgen twice
[MemCpyOpt] Only replace memcpy with bitcast if address spaces match
[ArgPromotion] Fix a truncated variable
[safestack] Disable stack coloring by default
merge-request.sh: Use https url for bugzilla
[ARM] Clear the constant pool cache on explicit .ltorg directives
[ARM] Temporarily disable globals promotion to constant pools to prevent miscompilation
[TLI] Add prototype checking for all remaining LibFuncs
[PPC] Properly update register save area offset
[PPC] When restoring R30 (PIC base pointer), mark it as <def>
Make library calls sensitive to regparm module flag (Fixes PR3997).
[GlobalMerge] Don't merge globals that may be preempted
[Support] Fix ErrorOr assertion when /proc/cpuinfo doesn't exist.
