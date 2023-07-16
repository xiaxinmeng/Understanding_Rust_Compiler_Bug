`
   Compiling alliteration-gen v0.1.0 (file:///tmp/test/nolib)
  time: 0.311; rss: 101MB       parsing
  time: 0.000; rss: 101MB       recursion limit
  time: 0.000; rss: 101MB       crate injection
  time: 0.000; rss: 101MB       plugin loading
  time: 0.000; rss: 101MB       plugin registration
  time: 0.044; rss: 124MB       expansion
  time: 0.000; rss: 124MB       maybe building test harness
  time: 0.001; rss: 124MB       maybe creating a macro crate
  time: 0.006; rss: 124MB       creating allocators
  time: 0.004; rss: 124MB       AST validation
  time: 0.006; rss: 128MB       name resolution
  time: 0.001; rss: 128MB       complete gated feature checking
  time: 0.017; rss: 146MB       lowering ast -> hir
  time: 0.018; rss: 146MB       early lint checks
warning: The output of `-Z time-llvm-passes` will only reflect timings of re-translated modules when used with incremental compilation
  time: 0.057; rss: 162MB       indexing hir
  time: 0.000; rss: 124MB       load query result cache
  time: 0.000; rss: 124MB       looking for entry point
  time: 0.000; rss: 124MB       looking for plugin registrar
  time: 0.001; rss: 124MB       loop checking
  time: 0.000; rss: 124MB       attribute checking
  time: 0.002; rss: 124MB       stability checking
  time: 0.019; rss: 139MB       type collecting
  time: 0.000; rss: 139MB       outlives testing
  time: 0.000; rss: 139MB       impl wf inference
  time: 0.000; rss: 139MB       coherence checking
  time: 0.000; rss: 139MB       variance testing
  time: 0.012; rss: 143MB       wf checking
  time: 0.000; rss: 143MB       item-types checking
  time: 1.599; rss: 204MB       item-bodies checking
  time: 0.105; rss: 211MB       rvalue promotion
  time: 0.034; rss: 211MB       privacy checking
  time: 0.002; rss: 211MB       intrinsic checking
  time: 0.002; rss: 211MB       match checking
  time: 0.003; rss: 211MB       liveness checking
  time: 1.246; rss: 423MB       borrow checking
  time: 0.000; rss: 423MB       MIR borrow checking
  time: 0.000; rss: 423MB       MIR effect checking
  time: 0.003; rss: 423MB       death checking
  time: 0.000; rss: 423MB       unused lib feature checking
  time: 0.098; rss: 423MB       lint checking
  time: 0.000; rss: 423MB       dumping chalk-like clauses
  time: 0.000; rss: 423MB       resolving dependency formats
    time: 0.000; rss: 423MB     write metadata
    time: 0.450; rss: 450MB     translation item collection
    time: 0.000; rss: 450MB     codegen unit partitioning
    time: 0.000; rss: 450MB     write allocator module
    time: 1.286; rss: 622MB     translate to LLVM IR
    time: 0.000; rss: 622MB     assert dep graph
    time: 0.000; rss: 622MB     serialize dep graph
  time: 1.739; rss: 622MB       translation
    time: 0.833; rss: 384MB     llvm function passes [alliteration_gen0]
    time: 9852.241; rss: 546MB  llvm module passes [alliteration_gen0]
    time: 1685.726; rss: 463MB  codegen passes [alliteration_gen0]
  time: 11539.432; rss: 163MB   LLVM passes
===-------------------------------------------------------------------------===
                              Register Allocation
===-------------------------------------------------------------------------===
  Total Execution Time: 0.0205 seconds (0.0205 wall clock)
   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.0173 ( 99.8%)   0.0032 (100.0%)   0.0205 ( 99.8%)   0.0205 ( 99.8%)  Seed Live Regs
   0.0000 (  0.2%)   0.0000 (  0.0%)   0.0000 (  0.2%)   0.0000 (  0.2%)  Evict
   0.0174 (100.0%)   0.0032 (100.0%)   0.0205 (100.0%)   0.0205 (100.0%)  Total
===-------------------------------------------------------------------------===
                      Instruction Selection and Scheduling
===-------------------------------------------------------------------------===
  Total Execution Time: 1535.6036 seconds (1536.6184 wall clock)
   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
  1529.1632 ( 99.6%)   0.2696 ( 73.8%)  1529.4328 ( 99.6%)  1530.4475 ( 99.6%)  DAG Combining 1
   2.3814 (  0.2%)   0.0135 (  3.7%)   2.3949 (  0.2%)   2.3950 (  0.2%)  DAG Combining 2
   1.9965 (  0.1%)   0.0300 (  8.2%)   2.0265 (  0.1%)   2.0265 (  0.1%)  Instruction Selection
   0.8134 (  0.1%)   0.0065 (  1.8%)   0.8200 (  0.1%)   0.8200 (  0.1%)  DAG Legalization
   0.4507 (  0.0%)   0.0232 (  6.3%)   0.4739 (  0.0%)   0.4739 (  0.0%)  Instruction Scheduling
   0.2092 (  0.0%)   0.0000 (  0.0%)   0.2092 (  0.0%)   0.2092 (  0.0%)  Type Legalization
   0.1781 (  0.0%)   0.0168 (  4.6%)   0.1949 (  0.0%)   0.1949 (  0.0%)  Instruction Creation
   0.0343 (  0.0%)   0.0000 (  0.0%)   0.0343 (  0.0%)   0.0343 (  0.0%)  Vector Legalization
   0.0111 (  0.0%)   0.0056 (  1.5%)   0.0167 (  0.0%)   0.0167 (  0.0%)  Instruction Scheduling Cleanup
   0.0004 (  0.0%)   0.0000 (  0.0%)   0.0004 (  0.0%)   0.0004 (  0.0%)  DAG Combining after legalize types
  1535.2383 (100.0%)   0.3653 (100.0%)  1535.6036 (100.0%)  1536.6184 (100.0%)  Total
===-------------------------------------------------------------------------===
                                 DWARF Emission
===-------------------------------------------------------------------------===
  Total Execution Time: 0.3086 seconds (0.3233 wall clock)
   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
   0.1348 ( 54.3%)   0.0351 ( 58.2%)   0.1699 ( 55.1%)   0.1843 ( 57.0%)  Debug Info Emission
   0.1115 ( 44.9%)   0.0247 ( 40.9%)   0.1362 ( 44.2%)   0.1366 ( 42.2%)  DWARF Exception Writer
   0.0019 (  0.8%)   0.0005 (  0.9%)   0.0024 (  0.8%)   0.0024 (  0.7%)  DWARF Debug Writer
   0.2482 (100.0%)   0.0603 (100.0%)   0.3086 (100.0%)   0.3233 (100.0%)  Total
===-------------------------------------------------------------------------===
                      ... Pass execution timing report ...
===-------------------------------------------------------------------------===
  Total Execution Time: 11517.6105 seconds (11536.0309 wall clock)
   ---User Time---   --System Time--   --User+System--   ---Wall Time---  --- Name ---
  9049.1177 ( 84.2%)  764.1362 ( 99.4%)  9813.2539 ( 85.2%)  9830.6488 ( 85.2%)  Combine redundant instructions
  1536.5964 ( 14.3%)   0.4161 (  0.1%)  1537.0125 ( 13.3%)  1538.0273 ( 13.3%)  Demanded bits analysis
  138.7323 (  1.3%)   3.3021 (  0.4%)  142.0344 (  1.2%)  142.1022 (  1.2%)  Machine Instruction Scheduler
  10.6652 (  0.1%)   0.0102 (  0.0%)  10.6754 (  0.1%)  10.6762 (  0.1%)  Dead Store Elimination
   2.7106 (  0.0%)   0.0500 (  0.0%)   2.7606 (  0.0%)   2.7606 (  0.0%)  SLP Vectorizer
   0.7723 (  0.0%)   0.0235 (  0.0%)   0.7958 (  0.0%)   0.7958 (  0.0%)  Combine redundant instructions
   0.7857 (  0.0%)   0.0069 (  0.0%)   0.7927 (  0.0%)   0.7927 (  0.0%)  Combine redundant instructions
   0.7482 (  0.0%)   0.0240 (  0.0%)   0.7723 (  0.0%)   0.7724 (  0.0%)  Combine redundant instructions
   0.7228 (  0.0%)   0.0233 (  0.0%)   0.7461 (  0.0%)   0.7461 (  0.0%)  Combine redundant instructions
   0.7324 (  0.0%)   0.0135 (  0.0%)   0.7459 (  0.0%)   0.7459 (  0.0%)  Combine redundant instructions
   0.5270 (  0.0%)   0.0265 (  0.0%)   0.5536 (  0.0%)   0.5536 (  0.0%)  X86 Assembly Printer
   0.4229 (  0.0%)   0.0252 (  0.0%)   0.4481 (  0.0%)   0.4481 (  0.0%)  Early CSE
   0.3849 (  0.0%)   0.0412 (  0.0%)   0.4260 (  0.0%)   0.4260 (  0.0%)  Global Value Numbering
   0.3855 (  0.0%)   0.0135 (  0.0%)   0.3991 (  0.0%)   0.3991 (  0.0%)  Combine redundant instructions
   0.3750 (  0.0%)   0.0133 (  0.0%)   0.3883 (  0.0%)   0.3883 (  0.0%)  Combine redundant instructions
   0.3343 (  0.0%)   0.0434 (  0.0%)   0.3776 (  0.0%)   0.3776 (  0.0%)  Called Value Propagation
   0.3229 (  0.0%)   0.0170 (  0.0%)   0.3399 (  0.0%)   0.3399 (  0.0%)  Early CSE w/ MemorySSA
   0.2652 (  0.0%)   0.0236 (  0.0%)   0.2887 (  0.0%)   0.2887 (  0.0%)  Value Propagation
....
    Finished release [optimized] target(s) in 11580.25 secs
