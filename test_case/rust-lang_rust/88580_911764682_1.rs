
===-------------------------------------------------------------------------===
                              Register Allocation
===-------------------------------------------------------------------------===
  Total Execution Time: 0.0001 seconds (0.0001 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.0000 (  0.0%)   0.0000 ( 49.4%)   0.0000 ( 44.9%)   0.0000 ( 47.0%)  Evict
   0.0000 (  0.0%)   0.0000 ( 32.1%)   0.0000 ( 29.2%)   0.0000 ( 28.3%)  Local Splitting
   0.0000 (100.0%)   0.0000 ( 18.5%)   0.0000 ( 25.8%)   0.0000 ( 24.7%)  Seed Live Regs
   0.0000 (100.0%)   0.0001 (100.0%)   0.0001 (100.0%)   0.0001 (100.0%)  Total

===-------------------------------------------------------------------------===
                      Instruction Selection and Scheduling
===-------------------------------------------------------------------------===
  Total Execution Time: 0.0027 seconds (0.0026 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.0002 ( 41.4%)   0.0006 ( 28.1%)   0.0008 ( 30.2%)   0.0008 ( 29.6%)  Instruction Selection
   0.0001 ( 19.5%)   0.0004 ( 19.5%)   0.0005 ( 19.5%)   0.0005 ( 20.0%)  DAG Combining 1
   0.0001 ( 13.4%)   0.0003 ( 13.2%)   0.0004 ( 13.3%)   0.0004 ( 13.4%)  Instruction Scheduling
   0.0000 ( 10.0%)   0.0002 ( 10.7%)   0.0003 ( 10.6%)   0.0003 ( 10.6%)  Instruction Creation
   0.0000 (  4.9%)   0.0002 ( 10.8%)   0.0003 (  9.9%)   0.0003 ( 10.1%)  DAG Combining 2
   0.0000 (  4.4%)   0.0002 (  7.3%)   0.0002 (  6.8%)   0.0002 (  6.9%)  DAG Legalization
   0.0000 (  4.1%)   0.0001 (  4.9%)   0.0001 (  4.8%)   0.0001 (  4.4%)  Type Legalization
   0.0000 (  0.7%)   0.0001 (  2.8%)   0.0001 (  2.5%)   0.0001 (  2.7%)  Vector Legalization
   0.0000 (  0.0%)   0.0000 (  1.8%)   0.0000 (  1.5%)   0.0000 (  1.5%)  DAG Combining after legalize types
   0.0000 (  1.7%)   0.0000 (  0.8%)   0.0000 (  1.0%)   0.0000 (  0.9%)  Instruction Scheduling Cleanup
   0.0004 (100.0%)   0.0023 (100.0%)   0.0027 (100.0%)   0.0026 (100.0%)  Total

===-------------------------------------------------------------------------===
                                 DWARF Emission
===-------------------------------------------------------------------------===
  Total Execution Time: 0.0001 seconds (0.0001 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.0000 ( 48.1%)   0.0001 ( 71.3%)   0.0001 ( 62.8%)   0.0001 ( 61.2%)  DWARF Exception Writer
   0.0000 ( 44.4%)   0.0000 ( 28.7%)   0.0001 ( 34.5%)   0.0000 ( 34.5%)  Debug Info Emission
   0.0000 (  7.4%)   0.0000 (  0.0%)   0.0000 (  2.7%)   0.0000 (  4.3%)  DWARF Debug Writer
   0.0001 (100.0%)   0.0001 (100.0%)   0.0001 (100.0%)   0.0001 (100.0%)  Total

===-------------------------------------------------------------------------===
                      ... Pass execution timing report ...
===-------------------------------------------------------------------------===
  Total Execution Time: 0.0583 seconds (0.0575 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.0028 (  7.0%)   0.0009 (  5.1%)   0.0037 (  6.4%)   0.0037 (  6.4%)  Function Integration/Inlining
   0.0001 (  0.3%)   0.0034 ( 18.4%)   0.0035 (  6.0%)   0.0035 (  6.1%)  X86 DAG->DAG Instruction Selection
   0.0029 (  7.2%)   0.0002 (  1.4%)   0.0031 (  5.4%)   0.0031 (  5.5%)  Dead Store Elimination
   0.0026 (  6.4%)   0.0004 (  2.0%)   0.0029 (  5.0%)   0.0029 (  5.1%)  Induction Variable Simplification
   0.0015 (  3.6%)   0.0004 (  2.0%)   0.0018 (  3.1%)   0.0018 (  3.1%)  Combine redundant instructions
   0.0014 (  3.5%)   0.0001 (  0.4%)   0.0015 (  2.5%)   0.0015 (  2.6%)  Machine Natural Loop Construction
   0.0011 (  2.8%)   0.0003 (  1.9%)   0.0015 (  2.5%)   0.0015 (  2.6%)  Combine redundant instructions
   0.0010 (  2.5%)   0.0004 (  1.9%)   0.0014 (  2.3%)   0.0014 (  2.3%)  SROA
   0.0010 (  2.4%)   0.0003 (  1.6%)   0.0012 (  2.1%)   0.0012 (  2.2%)  Combine redundant instructions
   0.0008 (  2.1%)   0.0003 (  1.5%)   0.0011 (  1.9%)   0.0011 (  1.9%)  X86 DAG->DAG Instruction Selection
   0.0008 (  2.0%)   0.0003 (  1.4%)   0.0011 (  1.8%)   0.0011 (  1.9%)  Combine redundant instructions
   0.0008 (  2.0%)   0.0002 (  1.3%)   0.0010 (  1.8%)   0.0010 (  1.8%)  Module Verifier
   0.0007 (  1.8%)   0.0003 (  1.6%)   0.0010 (  1.7%)   0.0010 (  1.7%)  SROA
   0.0008 (  1.9%)   0.0002 (  1.3%)   0.0010 (  1.7%)   0.0010 (  1.7%)  Deduce function attributes
   0.0007 (  1.7%)   0.0003 (  1.4%)   0.0009 (  1.6%)   0.0009 (  1.6%)  Combine redundant instructions
   0.0007 (  1.7%)   0.0002 (  1.2%)   0.0009 (  1.6%)   0.0009 (  1.5%)  Global Value Numbering
   0.0007 (  1.7%)   0.0002 (  1.1%)   0.0009 (  1.5%)   0.0009 (  1.5%)  Assumption Cache Tracker
   0.0006 (  1.5%)   0.0002 (  1.1%)   0.0008 (  1.4%)   0.0008 (  1.4%)  Early CSE
   0.0007 (  1.7%)   0.0001 (  0.6%)   0.0008 (  1.4%)   0.0008 (  1.4%)  Unroll loops
   0.0005 (  1.3%)   0.0002 (  0.9%)   0.0007 (  1.2%)   0.0007 (  1.2%)  Early CSE
   0.0005 (  1.3%)   0.0002 (  0.9%)   0.0007 (  1.2%)   0.0007 (  1.2%)  Global Value Numbering
   0.0000 (  0.1%)   0.0006 (  3.2%)   0.0006 (  1.1%)   0.0006 (  1.1%)  Machine Instruction Scheduler
   0.0004 (  0.9%)   0.0001 (  0.6%)   0.0005 (  0.8%)   0.0005 (  0.8%)  Value Propagation
   0.0000 (  0.1%)   0.0004 (  2.3%)   0.0004 (  0.8%)   0.0004 (  0.8%)  X86 Assembly Printer
   0.0003 (  0.8%)   0.0001 (  0.6%)   0.0004 (  0.7%)   0.0004 (  0.7%)  Remove unused exception handling info
   0.0003 (  0.8%)   0.0001 (  0.5%)   0.0004 (  0.7%)   0.0004 (  0.7%)  Expand Atomic instructions
   0.0003 (  0.7%)   0.0001 (  0.6%)   0.0004 (  0.6%)   0.0004 (  0.7%)  Create Garbage Collector Module Metadata
   0.0003 (  0.7%)   0.0001 (  0.5%)   0.0004 (  0.6%)   0.0004 (  0.6%)  Value Propagation
   0.0003 (  0.8%)   0.0001 (  0.4%)   0.0004 (  0.6%)   0.0004 (  0.6%)  Loop Invariant Code Motion
   0.0000 (  0.0%)   0.0004 (  2.0%)   0.0004 (  0.6%)   0.0004 (  0.6%)  Live Variable Analysis
   0.0003 (  0.6%)   0.0001 (  0.5%)   0.0003 (  0.6%)   0.0004 (  0.6%)  Aggressive Dead Code Elimination
   0.0000 (  0.1%)   0.0003 (  1.8%)   0.0004 (  0.6%)   0.0004 (  0.6%)  Greedy Register Allocator
   0.0000 (  0.1%)   0.0003 (  1.8%)   0.0004 (  0.6%)   0.0004 (  0.6%)  Block Frequency Analysis
   0.0003 (  0.6%)   0.0001 (  0.5%)   0.0004 (  0.6%)   0.0004 (  0.6%)  Simplify the CFG
   0.0003 (  0.7%)   0.0001 (  0.4%)   0.0003 (  0.6%)   0.0003 (  0.6%)  Jump Threading
   0.0003 (  0.6%)   0.0001 (  0.4%)   0.0003 (  0.6%)   0.0003 (  0.6%)  Tail Call Elimination
   0.0002 (  0.6%)   0.0001 (  0.4%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Simplify the CFG
   0.0002 (  0.6%)   0.0001 (  0.3%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Exception handling preparation
   0.0002 (  0.6%)   0.0001 (  0.4%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Machine Instruction Scheduler
   0.0002 (  0.5%)   0.0001 (  0.4%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Sparse Conditional Constant Propagation
   0.0002 (  0.6%)   0.0001 (  0.4%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Simplify the CFG
   0.0003 (  0.7%)   0.0000 (  0.0%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Profile summary info
   0.0002 (  0.5%)   0.0001 (  0.3%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Interprocedural Sparse Conditional Constant Propagation
   0.0002 (  0.5%)   0.0001 (  0.5%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Target Pass Configuration
   0.0002 (  0.5%)   0.0001 (  0.4%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Jump Threading
   0.0001 (  0.3%)   0.0001 (  0.7%)   0.0003 (  0.4%)   0.0003 (  0.5%)  Dominator Tree Construction
   0.0002 (  0.4%)   0.0001 (  0.6%)   0.0003 (  0.5%)   0.0003 (  0.5%)  Scalar Evolution Analysis
   0.0002 (  0.5%)   0.0001 (  0.4%)   0.0003 (  0.4%)   0.0003 (  0.4%)  Scalar Evolution Analysis
   0.0002 (  0.4%)   0.0001 (  0.3%)   0.0002 (  0.4%)   0.0003 (  0.4%)  Scalar Evolution Analysis
   0.0002 (  0.5%)   0.0001 (  0.4%)   0.0003 (  0.4%)   0.0003 (  0.4%)  Simplify the CFG
   0.0002 (  0.5%)   0.0000 (  0.2%)   0.0002 (  0.4%)   0.0002 (  0.4%)  Loop Invariant Code Motion
   0.0002 (  0.4%)   0.0001 (  0.3%)   0.0002 (  0.4%)   0.0002 (  0.4%)  Bit-Tracking Dead Code Elimination
   0.0002 (  0.6%)   0.0000 (  0.0%)   0.0002 (  0.4%)   0.0002 (  0.4%)  Combine redundant instructions
   0.0002 (  0.5%)   0.0000 (  0.2%)   0.0002 (  0.4%)   0.0002 (  0.4%)  Rotate Loops
   0.0002 (  0.5%)   0.0001 (  0.3%)   0.0002 (  0.4%)   0.0002 (  0.4%)  Dead Argument Elimination
   0.0001 (  0.4%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.4%)  Safe Stack instrumentation pass
   0.0002 (  0.4%)   0.0001 (  0.3%)   0.0002 (  0.4%)   0.0002 (  0.4%)  Natural Loop Information
   0.0002 (  0.4%)   0.0001 (  0.3%)   0.0002 (  0.4%)   0.0002 (  0.4%)  X86 Assembly Printer
   0.0001 (  0.3%)   0.0001 (  0.4%)   0.0002 (  0.3%)   0.0002 (  0.4%)  Natural Loop Information
   0.0001 (  0.3%)   0.0001 (  0.3%)   0.0002 (  0.3%)   0.0002 (  0.4%)  Post-Dominator Tree Construction
   0.0002 (  0.5%)   0.0000 (  0.0%)   0.0002 (  0.4%)   0.0002 (  0.4%)  Combine redundant instructions
   0.0001 (  0.4%)   0.0000 (  0.3%)   0.0002 (  0.3%)   0.0002 (  0.4%)  Dominator Tree Construction
   0.0001 (  0.2%)   0.0001 (  0.6%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Control Flow Optimizer
   0.0001 (  0.4%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Function Alias Analysis Results
   0.0009 (  2.3%)   0.0003 (  1.6%)   0.0012 (  2.1%)   0.0002 (  0.3%)  Scalar Evolution Analysis
   0.0001 (  0.3%)   0.0000 (  0.3%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0002 (  1.0%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Prologue/Epilogue Insertion & Frame Finalization
   0.0001 (  0.4%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Canonicalize natural loops
   0.0002 (  0.5%)   0.0000 (  0.0%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Combine redundant instructions
   0.0002 (  0.4%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Dominator Tree Construction
   0.0001 (  0.4%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Dominator Tree Construction
   0.0001 (  0.3%)   0.0000 (  0.3%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Dominator Tree Construction
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Module Verifier
   0.0001 (  0.3%)   0.0001 (  0.3%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Canonicalize natural loops
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Function Alias Analysis Results
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Function Alias Analysis Results
   0.0001 (  0.3%)   0.0000 (  0.3%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Dominator Tree Construction
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Module Verifier
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Canonicalize natural loops
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Function Alias Analysis Results
   0.0001 (  0.3%)   0.0000 (  0.3%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Dominator Tree Construction
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Dominator Tree Construction
   0.0001 (  0.2%)   0.0001 (  0.5%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Demanded bits analysis
   0.0000 (  0.0%)   0.0002 (  0.8%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Machine Common Subexpression Elimination
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Function Alias Analysis Results
   0.0001 (  0.3%)   0.0000 (  0.3%)   0.0002 (  0.3%)   0.0002 (  0.3%)  Dominator Tree Construction
   0.0001 (  0.4%)   0.0000 (  0.0%)   0.0001 (  0.2%)   0.0001 (  0.3%)  Demanded bits analysis
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0002 (  0.3%)   0.0001 (  0.3%)  Function Alias Analysis Results
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0001 (  0.3%)   0.0001 (  0.3%)  Function Alias Analysis Results
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0001 (  0.3%)   0.0001 (  0.3%)  Function Alias Analysis Results
   0.0001 (  0.3%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Lazy Value Information Analysis
   0.0001 (  0.3%)   0.0000 (  0.0%)   0.0001 (  0.2%)   0.0001 (  0.2%)  PGOIndirectCallPromotion
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Memory Dependence Analysis
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0001 (  0.6%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Machine Copy Propagation Pass
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Conditionally eliminate dead library calls
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Loop-Closed SSA Form Pass
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Lazy Value Information Analysis
   0.0000 (  0.0%)   0.0001 (  0.6%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Two-Address instruction pass
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Remove unreachable blocks from the CFG
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Memory Dependence Analysis
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Loop-Closed SSA Form Pass
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Live Variable Analysis
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Unswitch loops
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  MergedLoadStoreMotion
   0.0000 (  0.0%)   0.0001 (  0.6%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Merge disjoint stack slots
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Loop-Closed SSA Form Pass
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0001 (  0.5%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Live Interval Analysis
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Insert XRay ops
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Memory Dependence Analysis
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0001 (  0.2%)   0.0000 (  0.2%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Greedy Register Allocator
   0.0001 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.2%)  Lazy Branch Probability Analysis
   0.0001 (  0.2%)   0.0000 (  0.0%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Globals Alias Analysis
   0.0001 (  0.1%)   0.0000 (  0.2%)   0.0001 (  0.1%)   0.0001 (  0.2%)  Lazy Branch Probability Analysis
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.2%)   0.0001 (  0.2%)  Insert stack protectors
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Lower 'expect' Intrinsics
   0.0001 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Memory Dependence Analysis
   0.0001 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Optimization Remark Emitter
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Optimization Remark Emitter
   0.0001 (  0.2%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Recognize loop idioms
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  LCSSA Verifier
   0.0000 (  0.0%)   0.0001 (  0.4%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Peephole Optimizations
   0.0001 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  LCSSA Verifier
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  LCSSA Verifier
   0.0000 (  0.1%)   0.0000 (  0.2%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Machine Block Frequency Analysis
   0.0001 (  0.2%)   0.0000 (  0.0%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Remove redundant instructions
   0.0000 (  0.0%)   0.0001 (  0.3%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Remove dead machine instructions
   0.0000 (  0.0%)   0.0001 (  0.3%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Machine Natural Loop Construction
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Live Interval Analysis
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Loop Vectorization
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Natural Loop Information
   0.0000 (  0.1%)   0.0000 (  0.2%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Contiguously Lay Out Funclets
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Natural Loop Information
   0.0000 (  0.0%)   0.0000 (  0.3%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Virtual Register Rewriter
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0001 (  0.1%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Machine Block Frequency Analysis
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Virtual Register Rewriter
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  CallGraph Construction
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.1%)   0.0000 (  0.1%)  X86 LEA Optimize
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Machine Block Frequency Analysis
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Natural Loop Information
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Scalar Evolution Analysis
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Global Variable Optimizer
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Remove dead machine instructions
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Optimize machine instruction PHIs
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Delete dead loops
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Machine Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Implement the 'patchable-function' attribute
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Slot index numbering
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Dominator Tree Construction
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Prologue/Epilogue Insertion & Frame Finalization
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  MachineDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Dead Global Elimination
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.1%)  Simplify the CFG
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)  Post-RA pseudo instruction expansion pass
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0000 (  0.1%)  A No-Op Barrier Pass
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Slot index numbering
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)  MachineDominator Tree Construction
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Canonicalize natural loops
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Constant Hoisting
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.1%)  Remove dead machine instructions
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Rewrite Symbols
   0.0000 (  0.1%)   0.0000 (  0.1%)   0.0001 (  0.1%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachineDominator Tree Construction
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 Atom pad short functions
   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine InstCombiner
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachineDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Constant Hoisting
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Two-Address instruction pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  CodeGen Prepare
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Peephole Optimizations
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)  Machine Loop Invariant Code Motion
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Partially inline calls to library functions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Execution dependency fix
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Remove unreachable machine basic blocks
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Live Register Matrix
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 Optimize Call Frame
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Block Frequency Analysis
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Shadow Stack GC Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachinePostDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachineDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Eliminate PHI nodes for register allocation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Simple Register Coalescing
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scalar Evolution Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Block Frequency Analysis
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Float to int
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 pseudo instruction expansion pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Branch Probability Basic Block Placement
   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Unnamed pass: implement Pass::getPassName()
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Loop Invariant Code Motion
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Exception handling preparation
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine code sinking
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Expand Atomic instructions
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachineDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Debug Variable Analysis
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Transform Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 pseudo instruction expansion pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachinePostDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Local Dynamic TLS Access Clean-up
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Loop Invariant Code Motion
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Spill Code Placement Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  CallGraph Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Virtual Register Map
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Global Variable Optimizer
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 Byte/Word Instruction Fixup
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Block Frequency Analysis
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Tail Duplication
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Live Register Matrix
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Remove unreachable machine basic blocks
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dead Global Elimination
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 LEA Fixup
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Common Subexpression Elimination
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Interleaved Access Pass
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Transform Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Natural Loop Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine InstCombiner
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Natural Loop Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Tail Duplication
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachinePostDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  CallGraph Construction
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 Fixup SetCC
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Early If-Conversion
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Loop Invariant Code Motion
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Bundle Machine CFG Edges
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Distribution
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Live Stack Slot Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Load Elimination
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Live DEBUG_VALUE analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachinePostDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Spill Code Placement Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Remove dead machine instructions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 FP Stackifier
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Canonicalize natural loops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Trace Metrics
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Access Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Slot index numbering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Interleaved Access Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Analyze Machine Code For Garbage Collection
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Optimization Remark Emitter
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Slot index numbering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Eliminate PHI nodes for register allocation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Lazy Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Post RA top-down list latency scheduler
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Copy Propagation Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Rename Disconnected Subregister Components
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Natural Loop Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop-Closed SSA Form Pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  MachineDominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Loop Access Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  LCSSA Verifier
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  LCSSA Verifier
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Local Stack Slot Allocation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  LCSSA Verifier
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 WinAlloca Expander
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 LEA Optimize
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Lazy Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Partially inline calls to library functions
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Compressing EVEX instrs to VEX encoding when possible
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Alignment from assumptions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Natural Loop Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Debug Variable Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Expand ISel Pseudo-instructions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Lower Garbage Collection Instructions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Optimization Remark Emitter
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  LCSSA Verifier
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 vzeroupper inserter
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine code sinking
   0.0000 (  0.0%)   0.0000 (  0.1%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Detect Dead Lanes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 Optimize Call Frame
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Inserts calls to mcount-like functions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Virtual Register Map
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Shrink Wrapping analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Globals Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  LCSSA Verifier
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 vzeroupper inserter
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 Fixup SetCC
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Live Stack Slot Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Tail Duplication
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Stack Slot Coloring
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Merge disjoint stack slots
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Bundle Machine CFG Edges
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Tail Duplication
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Lower Garbage Collection Instructions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Bundle Machine CFG Edges
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Rename Disconnected Subregister Components
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Process Implicit Definitions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 FP Stackifier
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Branch Probability Basic Block Placement
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Post-RA pseudo instruction expansion pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Analyze Machine Code For Garbage Collection
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Early If-Conversion
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  StackMap Liveness Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 WinAlloca Expander
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Inserts calls to mcount-like functions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Live DEBUG_VALUE analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 Atom pad short functions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 LEA Fixup
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Compressing EVEX instrs to VEX encoding when possible
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Local Stack Slot Allocation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Detect Dead Lanes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  X86 PIC Global Base Reg Initialization
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Deduce function attributes in RPO
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Infer set function attributes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Pre-ISel Intrinsic Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Merge Duplicate Global Constants
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Pre-ISel Intrinsic Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Assumption Cache Tracker
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Strip Unused Function Prototypes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Eliminate Available Externally Globals
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Pre-ISel Intrinsic Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Rewrite Symbols
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Force set function attributes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Library Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scoped NoAlias Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Pass Configuration
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scoped NoAlias Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Module Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Assumption Cache Tracker
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scoped NoAlias Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Profile summary info
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Transform Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Type-Based Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Create Garbage Collector Module Metadata
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Module Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Library Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Library Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Module Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Type-Based Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Type-Based Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Branch Probability Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Create Garbage Collector Module Metadata
   0.0400 (100.0%)   0.0184 (100.0%)   0.0583 (100.0%)   0.0575 (100.0%)  Total
