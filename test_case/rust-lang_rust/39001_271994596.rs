
$ rustc +nightly -Ztime-passes -Ztime-llvm-passes -O --crate-type=lib --emit=asm xx.rs
time: 0.001	parsing
time: 0.000	recursion limit
time: 0.000	crate injection
time: 0.000	plugin loading
time: 0.000	plugin registration
time: 0.029	expansion
time: 0.000	maybe building test harness
time: 0.000	maybe creating a macro crate
time: 0.000	checking for inline asm in case the target doesn't support it
time: 0.000	complete gated feature checking
time: 0.000	early lint checks
time: 0.000	AST validation
time: 0.001	name resolution
time: 0.000	lowering ast -> hir
time: 0.000	indexing hir
time: 0.000	attribute checking
time: 0.000	language item collection
time: 0.000	lifetime resolution
time: 0.000	looking for entry point
time: 0.000	looking for plugin registrar
time: 0.000	region resolution
time: 0.000	loop checking
time: 0.000	static item recursion checking
time: 0.000	compute_incremental_hashes_map
time: 0.000	load_dep_graph
time: 0.000	stability index
time: 0.000	stability checking
time: 0.000	type collecting
time: 0.000	variance inference
time: 0.000	impl wf inference
time: 0.006	coherence checking
time: 0.000	wf checking
time: 0.000	item-types checking
time: 0.001	item-bodies checking
time: 0.000	drop-impl checking
time: 0.000	const checking
time: 0.000	privacy checking
time: 0.000	intrinsic checking
time: 0.000	effect checking
time: 0.000	match checking
time: 0.000	liveness checking
time: 0.000	rvalue checking
time: 0.000	MIR dump
  time: 0.000	SimplifyCfg
  time: 0.000	QualifyAndPromoteConstants
  time: 0.000	TypeckMir
  time: 0.000	SimplifyBranches
  time: 0.000	SimplifyCfg
time: 0.000	MIR cleanup and validation
time: 0.000	borrow checking
time: 0.000	reachability checking
time: 0.000	death checking
time: 0.000	unused lib feature checking
time: 0.000	lint checking
time: 0.000	resolving dependency formats
  time: 0.000	NoLandingPads
  time: 0.000	SimplifyCfg
  time: 0.000	EraseRegions
  time: 0.000	AddCallGuards
  time: 0.000	ElaborateDrops
  time: 0.000	NoLandingPads
  time: 0.000	SimplifyCfg
  time: 0.000	InstCombine
  time: 0.000	Deaggregator
  time: 0.000	CopyPropagation
  time: 0.000	SimplifyLocals
  time: 0.000	AddCallGuards
  time: 0.000	PreTrans
time: 0.000	MIR optimisations
  time: 0.000	write metadata
  time: 0.114	translation item collection
  time: 0.000	codegen unit partitioning
  time: 0.000	internalize symbols
time: 0.173	translation
time: 0.000	assert dep graph
time: 0.000	serialize dep graph
  time: 0.123	llvm function passes [0]
  time: 0.529	llvm module passes [0]
  time: 158.209	codegen passes [0]
  time: 0.000	codegen passes [0]
===-------------------------------------------------------------------------===
                              Register Allocation
===-------------------------------------------------------------------------===
  Total Execution Time: 0.1732 seconds (0.1735 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.1081 ( 68.5%)   0.0102 ( 66.4%)   0.1183 ( 68.3%)   0.1186 ( 68.3%)  Evict
   0.0453 ( 28.7%)   0.0030 ( 19.8%)   0.0484 ( 27.9%)   0.0483 ( 27.8%)  Spiller
   0.0041 (  2.6%)   0.0021 ( 13.7%)   0.0062 (  3.6%)   0.0063 (  3.7%)  Local Splitting
   0.0003 (  0.2%)   0.0000 (  0.0%)   0.0003 (  0.2%)   0.0003 (  0.2%)  Seed Live Regs
   0.1579 (100.0%)   0.0153 (100.0%)   0.1732 (100.0%)   0.1735 (100.0%)  Total

===-------------------------------------------------------------------------===
                      Instruction Selection and Scheduling
===-------------------------------------------------------------------------===
  Total Execution Time: 151.3933 seconds (155.0033 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
  148.1731 ( 98.3%)   0.6826 ( 97.5%)  148.8557 ( 98.3%)  152.4434 ( 98.3%)  DAG Combining 1
   2.4343 (  1.6%)   0.0116 (  1.7%)   2.4459 (  1.6%)   2.4680 (  1.6%)  Instruction Scheduling
   0.0424 (  0.0%)   0.0030 (  0.4%)   0.0454 (  0.0%)   0.0456 (  0.0%)  Instruction Selection
   0.0265 (  0.0%)   0.0008 (  0.1%)   0.0273 (  0.0%)   0.0274 (  0.0%)  DAG Combining 2
   0.0064 (  0.0%)   0.0021 (  0.3%)   0.0085 (  0.0%)   0.0085 (  0.0%)  Instruction Creation
   0.0066 (  0.0%)   0.0003 (  0.0%)   0.0069 (  0.0%)   0.0069 (  0.0%)  DAG Legalization
   0.0033 (  0.0%)   0.0000 (  0.0%)   0.0034 (  0.0%)   0.0034 (  0.0%)  Type Legalization
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  Instruction Scheduling Cleanup
   0.0001 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)   0.0001 (  0.0%)  Vector Legalization
  150.6930 (100.0%)   0.7003 (100.0%)  151.3933 (100.0%)  155.0033 (100.0%)  Total

===-------------------------------------------------------------------------===
                                 DWARF Emission
===-------------------------------------------------------------------------===
  Total Execution Time: 0.0000 seconds (0.0000 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.0000 ( 47.8%)   0.0000 ( 60.0%)   0.0000 ( 52.6%)   0.0000 ( 55.8%)  Debug Info Emission
   0.0000 ( 43.5%)   0.0000 ( 26.7%)   0.0000 ( 36.8%)   0.0000 ( 35.9%)  DWARF Exception Writer
   0.0000 (  8.7%)   0.0000 ( 13.3%)   0.0000 ( 10.5%)   0.0000 (  8.3%)  DWARF Debug Writer
   0.0000 (100.0%)   0.0000 (100.0%)   0.0000 (100.0%)   0.0000 (100.0%)  Total

===-------------------------------------------------------------------------===
                      ... Pass execution timing report ...
===-------------------------------------------------------------------------===
  Total Execution Time: 154.7966 seconds (158.8321 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
  151.8207 ( 99.0%)   1.4273 ( 94.9%)  153.2481 ( 99.0%)  157.2726 ( 99.0%)  Dominator Tree Construction
   0.2495 (  0.2%)   0.0293 (  1.9%)   0.2788 (  0.2%)   0.2833 (  0.2%)  Greedy Register Allocator
   0.2654 (  0.2%)   0.0033 (  0.2%)   0.2687 (  0.2%)   0.2697 (  0.2%)  Machine Instruction Scheduler
   0.2637 (  0.2%)   0.0008 (  0.1%)   0.2645 (  0.2%)   0.2654 (  0.2%)  Stack Slot Coloring
   0.2051 (  0.1%)   0.0017 (  0.1%)   0.2069 (  0.1%)   0.2092 (  0.1%)  Global Value Numbering
   0.1993 (  0.1%)   0.0023 (  0.2%)   0.2016 (  0.1%)   0.2020 (  0.1%)  Global Value Numbering
   0.1031 (  0.1%)   0.0061 (  0.4%)   0.1092 (  0.1%)   0.1094 (  0.1%)  SROA
   0.0016 (  0.0%)   0.0166 (  1.1%)   0.0181 (  0.0%)   0.0182 (  0.0%)  Machine Function Analysis
   0.0052 (  0.0%)   0.0094 (  0.6%)   0.0146 (  0.0%)   0.0147 (  0.0%)  Live Interval Analysis
   0.0125 (  0.0%)   0.0002 (  0.0%)   0.0127 (  0.0%)   0.0128 (  0.0%)  Combine redundant instructions
   0.0103 (  0.0%)   0.0010 (  0.1%)   0.0114 (  0.0%)   0.0114 (  0.0%)  X86 Assembly / Object Emitter
   0.0096 (  0.0%)   0.0003 (  0.0%)   0.0099 (  0.0%)   0.0099 (  0.0%)  Combine redundant instructions
   0.0095 (  0.0%)   0.0002 (  0.0%)   0.0097 (  0.0%)   0.0099 (  0.0%)  Early CSE
   0.0090 (  0.0%)   0.0002 (  0.0%)   0.0092 (  0.0%)   0.0092 (  0.0%)  Early CSE
   0.0084 (  0.0%)   0.0005 (  0.0%)   0.0089 (  0.0%)   0.0089 (  0.0%)  Combine redundant instructions
   0.0082 (  0.0%)   0.0006 (  0.0%)   0.0088 (  0.0%)   0.0088 (  0.0%)  Combine redundant instructions
   0.0076 (  0.0%)   0.0003 (  0.0%)   0.0078 (  0.0%)   0.0078 (  0.0%)  Combine redundant instructions
   0.0076 (  0.0%)   0.0001 (  0.0%)   0.0077 (  0.0%)   0.0077 (  0.0%)  Combine redundant instructions
   0.0074 (  0.0%)   0.0001 (  0.0%)   0.0075 (  0.0%)   0.0076 (  0.0%)  Combine redundant instructions
   0.0074 (  0.0%)   0.0000 (  0.0%)   0.0075 (  0.0%)   0.0075 (  0.0%)  Combine redundant instructions
   0.0065 (  0.0%)   0.0001 (  0.0%)   0.0066 (  0.0%)   0.0066 (  0.0%)  Prologue/Epilogue Insertion & Frame Finalization
   0.0045 (  0.0%)   0.0001 (  0.0%)   0.0046 (  0.0%)   0.0047 (  0.0%)  Virtual Register Rewriter
   0.0044 (  0.0%)   0.0001 (  0.0%)   0.0045 (  0.0%)   0.0045 (  0.0%)  CodeGen Prepare
   0.0037 (  0.0%)   0.0003 (  0.0%)   0.0039 (  0.0%)   0.0044 (  0.0%)  Live Variable Analysis
   0.0038 (  0.0%)   0.0000 (  0.0%)   0.0039 (  0.0%)   0.0039 (  0.0%)  Value Propagation
   0.0034 (  0.0%)   0.0001 (  0.0%)   0.0035 (  0.0%)   0.0035 (  0.0%)  Function Integration/Inlining
   0.0030 (  0.0%)   0.0004 (  0.0%)   0.0034 (  0.0%)   0.0034 (  0.0%)  Natural Loop Information
   0.0029 (  0.0%)   0.0003 (  0.0%)   0.0033 (  0.0%)   0.0033 (  0.0%)  Interprocedural Sparse Conditional Constant Propagation
   0.0031 (  0.0%)   0.0000 (  0.0%)   0.0031 (  0.0%)   0.0033 (  0.0%)  Module Verifier
   0.0024 (  0.0%)   0.0003 (  0.0%)   0.0027 (  0.0%)   0.0027 (  0.0%)  Sparse Conditional Constant Propagation
   0.0027 (  0.0%)   0.0000 (  0.0%)   0.0027 (  0.0%)   0.0027 (  0.0%)  Value Propagation
   0.0025 (  0.0%)   0.0000 (  0.0%)   0.0025 (  0.0%)   0.0025 (  0.0%)  Machine Copy Propagation Pass
   0.0023 (  0.0%)   0.0002 (  0.0%)   0.0025 (  0.0%)   0.0025 (  0.0%)  Jump Threading
   0.0021 (  0.0%)   0.0000 (  0.0%)   0.0021 (  0.0%)   0.0021 (  0.0%)  Module Verifier
   0.0020 (  0.0%)   0.0000 (  0.0%)   0.0021 (  0.0%)   0.0020 (  0.0%)  X86 Byte/Word Instruction Fixup
   0.0019 (  0.0%)   0.0001 (  0.0%)   0.0020 (  0.0%)   0.0020 (  0.0%)  Bit-Tracking Dead Code Elimination
   0.0016 (  0.0%)   0.0002 (  0.0%)   0.0019 (  0.0%)   0.0019 (  0.0%)  Slot index numbering
   0.0016 (  0.0%)   0.0000 (  0.0%)   0.0017 (  0.0%)   0.0017 (  0.0%)  Reassociate expressions
   0.0016 (  0.0%)   0.0001 (  0.0%)   0.0016 (  0.0%)   0.0016 (  0.0%)  Deduce function attributes
   0.0014 (  0.0%)   0.0001 (  0.0%)   0.0016 (  0.0%)   0.0016 (  0.0%)  X86 LEA Optimize
   0.0012 (  0.0%)   0.0000 (  0.0%)   0.0012 (  0.0%)   0.0013 (  0.0%)  Remove dead machine instructions
   0.0013 (  0.0%)   0.0000 (  0.0%)   0.0013 (  0.0%)   0.0013 (  0.0%)  Remove redundant instructions
   0.0012 (  0.0%)   0.0001 (  0.0%)   0.0013 (  0.0%)   0.0013 (  0.0%)  Two-Address instruction pass
   0.0012 (  0.0%)   0.0000 (  0.0%)   0.0012 (  0.0%)   0.0012 (  0.0%)  Aggressive Dead Code Elimination
   0.0010 (  0.0%)   0.0000 (  0.0%)   0.0010 (  0.0%)   0.0010 (  0.0%)  Tail Call Elimination
   0.0010 (  0.0%)   0.0000 (  0.0%)   0.0010 (  0.0%)   0.0010 (  0.0%)  Remove dead machine instructions
   0.0009 (  0.0%)   0.0001 (  0.0%)   0.0010 (  0.0%)   0.0010 (  0.0%)  Live Stack Slot Analysis
   0.0007 (  0.0%)   0.0000 (  0.0%)   0.0008 (  0.0%)   0.0008 (  0.0%)  MemCpy Optimization
   0.0007 (  0.0%)   0.0000 (  0.0%)   0.0007 (  0.0%)   0.0008 (  0.0%)  Machine Common Subexpression Elimination
   0.0007 (  0.0%)   0.0000 (  0.0%)   0.0007 (  0.0%)   0.0007 (  0.0%)  Remove unused exception handling info
   0.0007 (  0.0%)   0.0000 (  0.0%)   0.0007 (  0.0%)   0.0007 (  0.0%)  CallGraph Construction
   0.0007 (  0.0%)   0.0000 (  0.0%)   0.0007 (  0.0%)   0.0007 (  0.0%)  Post-RA pseudo instruction expansion pass
   0.0006 (  0.0%)   0.0000 (  0.0%)   0.0006 (  0.0%)   0.0006 (  0.0%)  Simplify the CFG
   0.0006 (  0.0%)   0.0000 (  0.0%)   0.0006 (  0.0%)   0.0006 (  0.0%)  Constant Hoisting
   0.0006 (  0.0%)   0.0000 (  0.0%)   0.0006 (  0.0%)   0.0006 (  0.0%)  Dead Global Elimination
   0.0005 (  0.0%)   0.0000 (  0.0%)   0.0006 (  0.0%)   0.0006 (  0.0%)  Jump Threading
   0.0005 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.0%)   0.0005 (  0.0%)  X86 Optimize Call Frame
   0.0005 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.0%)   0.0005 (  0.0%)  Lower 'expect' Intrinsics
   0.0005 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.0%)   0.0005 (  0.0%)  SROA
   0.0005 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.0%)   0.0005 (  0.0%)  X86 pseudo instruction expansion pass
   0.0004 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  Dead Store Elimination
   0.0004 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  Promote Memory to Register
   0.0004 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  X86 LEA Fixup
   0.0003 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  Simplify the CFG
   0.0004 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  Expand Atomic instructions
   0.0004 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  Simplify the CFG
   0.0004 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  Lazy Block Frequency Analysis
   0.0004 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  X86 Fixup SetCC
   0.0002 (  0.0%)   0.0001 (  0.0%)   0.0003 (  0.0%)   0.0003 (  0.0%)  Insert stack protectors
   0.0003 (  0.0%)   0.0000 (  0.0%)   0.0003 (  0.0%)   0.0003 (  0.0%)  PGOIndirectCallPromotion
   0.0003 (  0.0%)   0.0000 (  0.0%)   0.0003 (  0.0%)   0.0003 (  0.0%)  Simplify the CFG
   0.0003 (  0.0%)   0.0000 (  0.0%)   0.0003 (  0.0%)   0.0003 (  0.0%)  Debug Variable Analysis
   0.0002 (  0.0%)   0.0001 (  0.0%)   0.0003 (  0.0%)   0.0003 (  0.0%)  Globals Alias Analysis
   0.0002 (  0.0%)   0.0001 (  0.0%)   0.0003 (  0.0%)   0.0003 (  0.0%)  Module Verifier
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0003 (  0.0%)   0.0003 (  0.0%)  Machine InstCombiner
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  Simplify the CFG
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  Globals Alias Analysis
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  CallGraph Construction
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  Simplify the CFG
   0.0001 (  0.0%)   0.0001 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  Demanded bits analysis
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  Partially inline calls to library functions
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  Float to int
   0.0002 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)   0.0002 (  0.0%)  Simplify the CFG
   0.0001 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)   0.0001 (  0.0%)  Process Implicit Definitions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)   0.0001 (  0.0%)  Scalar Evolution Analysis
   0.0001 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)   0.0001 (  0.0%)  Dead Argument Elimination
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)   0.0001 (  0.0%)  MachineDominator Tree Construction
   0.0001 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)   0.0001 (  0.0%)  Live Register Matrix
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)   0.0001 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)   0.0001 (  0.0%)  Natural Loop Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)  Machine Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachineDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Natural Loop Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Trace Metrics
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Natural Loop Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Natural Loop Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Profile summary info
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Global Variable Optimizer
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachineDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Shrink Wrapping analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Alignment from assumptions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Bundle Machine CFG Edges
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachinePostDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Lazy Value Information Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Access Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Memory Dependence Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Natural Loop Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Vectorization
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Infer set function attributes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Tail Duplication
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Assumption Cache Tracker
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Lazy Value Information Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Loop Invariant Code Motion
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Remove unreachable machine basic blocks
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Speculatively execute instructions if target has divergent branches
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Virtual Register Map
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachinePostDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Post RA top-down list latency scheduler
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Memory Dependence Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Eliminate PHI nodes for register allocation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Library Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Early If-Conversion
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Natural Loop Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Memory Dependence Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MergedLoadStoreMotion
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Memory Dependence Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Loop Invariant Code Motion
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachineDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Access Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Detect Dead Lanes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Local Stack Slot Allocation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 WinAlloca Expander
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Strip Unused Function Prototypes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Remove unreachable blocks from the CFG
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Distribition
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Live DEBUG_VALUE analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Natural Loop Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Pre-ISel Intrinsic Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Access Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Deduce function attributes in RPO
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Optimize machine instruction PHIs
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Optimization Remark Emitter
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Load Elimination
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Bundle Machine CFG Edges
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Shadow Stack GC Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Rename Disconnected Subregister Components
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Force set function attributes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  StackMap Liveness Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 FP Stackifier
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Merge Duplicate Global Constants
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Demanded bits analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Analyze Machine Code For Garbage Collection
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Create Garbage Collector Module Metadata
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Implement the 'patchable-function' attribute
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Assumption Cache Tracker
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Tail Duplication
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Transform Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Lower Garbage Collection Instructions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Eliminate Available Externally Globals
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  A No-Op Barrier Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 vzeroupper inserter
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scoped NoAlias Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scoped NoAlias Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Type-Based Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Type-Based Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Pass Configuration
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Library Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Module Information
  153.2922 (100.0%)   1.5044 (100.0%)  154.7966 (100.0%)  158.8321 (100.0%)  Total

time: 158.872	LLVM passes
time: 0.000	serialize work products
