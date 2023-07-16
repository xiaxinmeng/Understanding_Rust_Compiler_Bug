
[chrivers@helios]~/git/artemis/oxide #cargo rustc --bin oxide -- -Ztime-passes -Ztime-llvm-passes                  [pts/0 # 11945H 1J 0R # 2016-09-23 18:27:25 # 0+08:11:05 # 0.65 1.06 0.86]
   Compiling oxide v0.1.0 (file:///home/chrivers/git/artemis/oxide)
warning: this feature has been stable since 1.13.0. Attribute no longer needed, #[warn(stable_features)] on by default
 --> src/lib.rs:2:12
  |
2 | #![feature(type_macros)]
  |            ^^^^^^^^^^^

time: 0.002; rss: 49MB  parsing
time: 0.000; rss: 49MB  recursion limit
time: 0.000; rss: 49MB  crate injection
time: 0.000; rss: 49MB  plugin loading
time: 0.000; rss: 49MB  plugin registration
time: 0.024; rss: 74MB  expansion
time: 0.000; rss: 74MB  maybe building test harness
time: 0.000; rss: 74MB  maybe creating a macro crate
time: 0.000; rss: 74MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 74MB  complete gated feature checking
time: 0.000; rss: 74MB  collecting defs
time: 0.028; rss: 93MB  external crate/lib resolution
time: 0.000; rss: 93MB  early lint checks
time: 0.000; rss: 93MB  AST validation
time: 0.002; rss: 93MB  name resolution
time: 0.000; rss: 94MB  lowering ast -> hir
time: 0.000; rss: 94MB  indexing hir
time: 0.000; rss: 94MB  attribute checking
time: 0.000; rss: 94MB  language item collection
time: 0.000; rss: 94MB  lifetime resolution
time: 0.000; rss: 94MB  looking for entry point
time: 0.000; rss: 94MB  looking for plugin registrar
time: 0.000; rss: 94MB  region resolution
time: 0.000; rss: 94MB  loop checking
time: 0.000; rss: 94MB  static item recursion checking
time: 0.002; rss: 94MB  compute_incremental_hashes_map
time: 0.000; rss: 94MB  load_dep_graph
time: 0.003; rss: 96MB  type collecting
time: 0.000; rss: 96MB  variance inference
time: 0.007; rss: 96MB  coherence checking
time: 0.009; rss: 98MB  wf checking
time: 0.002; rss: 98MB  item-types checking
time: 0.056; rss: 104MB item-bodies checking
time: 0.000; rss: 104MB drop-impl checking
time: 0.002; rss: 104MB const checking
time: 0.000; rss: 104MB privacy checking
time: 0.000; rss: 104MB stability index
time: 0.000; rss: 104MB intrinsic checking
time: 0.000; rss: 104MB effect checking
time: 0.000; rss: 104MB match checking
time: 0.000; rss: 105MB liveness checking
time: 0.001; rss: 105MB rvalue checking
time: 0.003; rss: 106MB MIR dump
  time: 0.000; rss: 106MB       SimplifyCfg
  time: 0.001; rss: 106MB       QualifyAndPromoteConstants
  time: 0.001; rss: 106MB       TypeckMir
  time: 0.000; rss: 106MB       SimplifyBranches
  time: 0.000; rss: 106MB       SimplifyCfg
time: 0.002; rss: 106MB MIR passes
time: 0.005; rss: 108MB borrow checking
time: 0.000; rss: 108MB reachability checking
time: 0.000; rss: 108MB death checking
time: 0.000; rss: 108MB stability checking
time: 0.000; rss: 108MB unused lib feature checking
time: 0.006; rss: 108MB lint checking
time: 0.004; rss: 108MB resolving dependency formats
  time: 0.000; rss: 108MB       NoLandingPads
  time: 0.000; rss: 108MB       SimplifyCfg
  time: 0.001; rss: 108MB       EraseRegions
  time: 0.000; rss: 108MB       AddCallGuards
  time: 0.001; rss: 108MB       ElaborateDrops
  time: 0.000; rss: 108MB       NoLandingPads
  time: 0.000; rss: 108MB       SimplifyCfg
  time: 0.000; rss: 108MB       InstCombine
  time: 0.000; rss: 108MB       Deaggregator
  time: 0.005; rss: 108MB       CopyPropagation
  time: 0.000; rss: 108MB       AddCallGuards
  time: 0.000; rss: 108MB       PreTrans
time: 0.008; rss: 108MB Prepare MIR codegen passes
  time: 0.000; rss: 108MB       write metadata
  time: 0.045; rss: 114MB       translation item collection
  time: 0.004; rss: 114MB       codegen unit partitioning
  time: 0.004; rss: 127MB       internalize symbols
time: 0.290; rss: 127MB translation
time: 0.000; rss: 127MB assert dep graph
time: 0.000; rss: 127MB serialize dep graph
  time: 0.026; rss: 115MB       llvm function passes [0]
  time: 0.010; rss: 116MB       llvm module passes [0]
  time: 0.583; rss: 120MB       codegen passes [0]
  time: 0.000; rss: 120MB       codegen passes [0]
===-------------------------------------------------------------------------===
                      Instruction Selection and Scheduling
===-------------------------------------------------------------------------===
  Total Execution Time: 0.1240 seconds (0.1095 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.0200 ( 18.5%)   0.0000 (  0.0%)   0.0200 ( 16.1%)   0.0304 ( 27.8%)  Instruction Selection
   0.0160 ( 14.8%)   0.0040 ( 25.0%)   0.0200 ( 16.1%)   0.0185 ( 16.9%)  Instruction Scheduling
   0.0160 ( 14.8%)   0.0080 ( 50.0%)   0.0240 ( 19.4%)   0.0155 ( 14.1%)  DAG Combining 1
   0.0120 ( 11.1%)   0.0000 (  0.0%)   0.0120 (  9.7%)   0.0104 (  9.5%)  DAG Combining 2
   0.0160 ( 14.8%)   0.0000 (  0.0%)   0.0160 ( 12.9%)   0.0104 (  9.5%)  Instruction Creation
   0.0120 ( 11.1%)   0.0040 ( 25.0%)   0.0160 ( 12.9%)   0.0093 (  8.5%)  DAG Legalization
   0.0080 (  7.4%)   0.0000 (  0.0%)   0.0080 (  6.5%)   0.0084 (  7.7%)  Type Legalization
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0033 (  3.0%)  DAG Combining after legalize types
   0.0080 (  7.4%)   0.0000 (  0.0%)   0.0080 (  6.5%)   0.0017 (  1.6%)  Vector Legalization
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0016 (  1.4%)  Instruction Scheduling Cleanup
   0.1080 (100.0%)   0.0160 (100.0%)   0.1240 (100.0%)   0.1095 (100.0%)  Total

===-------------------------------------------------------------------------===
                                 DWARF Emission
===-------------------------------------------------------------------------===
  Total Execution Time: 0.0360 seconds (0.0622 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.0160 ( 57.1%)   0.0080 (100.0%)   0.0240 ( 66.7%)   0.0423 ( 68.0%)  Debug Info Emission
   0.0120 ( 42.9%)   0.0000 (  0.0%)   0.0120 ( 33.3%)   0.0194 ( 31.2%)  DWARF Exception Writer
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.8%)  DWARF Debug Writer
   0.0280 (100.0%)   0.0080 (100.0%)   0.0360 (100.0%)   0.0622 (100.0%)  Total

===-------------------------------------------------------------------------===
                      ... Pass execution timing report ...
===-------------------------------------------------------------------------===
  Total Execution Time: 0.5400 seconds (0.5185 wall clock)

   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.2360 ( 50.4%)   0.0280 ( 38.9%)   0.2640 ( 48.9%)   0.2088 ( 40.3%)  X86 DAG->DAG Instruction Selection
   0.0760 ( 16.2%)   0.0200 ( 27.8%)   0.0960 ( 17.8%)   0.1273 ( 24.5%)  X86 Assembly / Object Emitter
   0.0200 (  4.3%)   0.0000 (  0.0%)   0.0200 (  3.7%)   0.0294 (  5.7%)  Module Verifier
   0.0280 (  6.0%)   0.0000 (  0.0%)   0.0280 (  5.2%)   0.0245 (  4.7%)  Module Verifier
   0.0240 (  5.1%)   0.0040 (  5.6%)   0.0280 (  5.2%)   0.0228 (  4.4%)  Module Verifier
   0.0040 (  0.9%)   0.0000 (  0.0%)   0.0040 (  0.7%)   0.0147 (  2.8%)  Prologue/Epilogue Insertion & Frame Finalization
   0.0160 (  3.4%)   0.0000 (  0.0%)   0.0160 (  3.0%)   0.0129 (  2.5%)  Fast Register Allocator
   0.0120 (  2.6%)   0.0000 (  0.0%)   0.0120 (  2.2%)   0.0082 (  1.6%)  Machine Function Analysis
   0.0040 (  0.9%)   0.0000 (  0.0%)   0.0040 (  0.7%)   0.0073 (  1.4%)  Inliner for always_inline functions
   0.0160 (  3.4%)   0.0000 (  0.0%)   0.0160 (  3.0%)   0.0070 (  1.4%)  Insert stack protectors
   0.0040 (  0.9%)   0.0000 (  0.0%)   0.0040 (  0.7%)   0.0065 (  1.3%)  Live DEBUG_VALUE analysis
   0.0080 (  1.7%)   0.0040 (  5.6%)   0.0120 (  2.2%)   0.0049 (  0.9%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0080 ( 11.1%)   0.0080 (  1.5%)   0.0046 (  0.9%)  Two-Address instruction pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0038 (  0.7%)  Scalar Evolution Analysis
   0.0040 (  0.9%)   0.0000 (  0.0%)   0.0040 (  0.7%)   0.0035 (  0.7%)  Natural Loop Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0033 (  0.6%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0029 (  0.6%)  Dominator Tree Construction
   0.0040 (  0.9%)   0.0000 (  0.0%)   0.0040 (  0.7%)   0.0027 (  0.5%)  Function Alias Analysis Results
   0.0000 (  0.0%)   0.0040 (  5.6%)   0.0040 (  0.7%)   0.0026 (  0.5%)  Dominator Tree Construction
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0024 (  0.5%)  Expand Atomic instructions
   0.0080 (  1.7%)   0.0000 (  0.0%)   0.0080 (  1.5%)   0.0022 (  0.4%)  CallGraph Construction
   0.0040 (  0.9%)   0.0000 (  0.0%)   0.0040 (  0.7%)   0.0019 (  0.4%)  Exception handling preparation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0014 (  0.3%)  Post-RA pseudo instruction expansion pass
   0.0000 (  0.0%)   0.0040 (  5.6%)   0.0040 (  0.7%)   0.0012 (  0.2%)  Remove unreachable blocks from the CFG
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0012 (  0.2%)  X86 pseudo instruction expansion pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0011 (  0.2%)  Basic Alias Analysis (stateless AA impl)
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0011 (  0.2%)  Bundle Machine CFG Edges
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0009 (  0.2%)  Eliminate PHI nodes for register allocation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0007 (  0.1%)  Expand ISel Pseudo-instructions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0006 (  0.1%)  Insert XRay ops
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.1%)  Contiguously Lay Out Funclets
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.1%)  X86 FP Stackifier
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.1%)  Implement the 'patchable-function' attribute
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.1%)  X86 PIC Global Base Reg Initialization
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0005 (  0.1%)  Local Stack Slot Allocation
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.1%)  X86 WinAlloca Expander
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.1%)  Analyze Machine Code For Garbage Collection
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.1%)  StackMap Liveness Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.1%)  X86 vzeroupper inserter
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.1%)  Safe Stack instrumentation pass
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.1%)  Shadow Stack GC Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.1%)  Lower Garbage Collection Instructions
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0002 (  0.0%)  Create Garbage Collector Module Metadata
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)  Assumption Cache Tracker
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0001 (  0.0%)  Pre-ISel Intrinsic Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Assumption Cache Tracker
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Library Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Library Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Rewrite Symbols
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Create Garbage Collector Module Metadata
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Assumption Cache Tracker
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Force set function attributes
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Type-Based Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Transform Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Target Transform Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scoped NoAlias Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Scoped NoAlias Alias Analysis
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Rewrite Symbols
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Profile summary info
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Pre-ISel Intrinsic Lowering
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Module Information
   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)   0.0000 (  0.0%)  Machine Module Information
   0.4680 (100.0%)   0.0720 (100.0%)   0.5400 (100.0%)   0.5185 (100.0%)  Total

time: 0.622; rss: 120MB LLVM passes
time: 0.000; rss: 120MB serialize work products
  time: 0.456; rss: 109MB       running linker
time: 0.458; rss: 109MB linking
    Finished debug [unoptimized + debuginfo] target(s) in 21.80 secs
