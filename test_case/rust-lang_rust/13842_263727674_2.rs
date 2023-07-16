text
$ for x in short medium long; do echo $x; time cargo rustc -- --cfg $x -Z time-passes; done
short
   Compiling testingtesting v0.1.0 (file:///C:/Users/steve/tmp/testingtesting)
time: 0.000; rss: 25MB  parsing
time: 0.000; rss: 25MB  recursion limit
time: 0.000; rss: 25MB  crate injection
time: 0.005; rss: 30MB  plugin loading
time: 0.000; rss: 30MB  plugin registration
time: 0.102; rss: 75MB  expansion
time: 0.000; rss: 75MB  maybe building test harness
time: 0.000; rss: 75MB  maybe creating a macro crate
time: 0.000; rss: 75MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 75MB  complete gated feature checking
time: 0.000; rss: 75MB  early lint checks
time: 0.000; rss: 75MB  AST validation
time: 0.008; rss: 75MB  name resolution
time: 0.002; rss: 75MB  lowering ast -> hir
time: 0.000; rss: 75MB  indexing hir
time: 0.000; rss: 75MB  attribute checking
time: 0.000; rss: 75MB  language item collection
time: 0.000; rss: 75MB  lifetime resolution
time: 0.000; rss: 75MB  looking for entry point
time: 0.000; rss: 75MB  looking for plugin registrar
time: 0.000; rss: 75MB  region resolution
time: 0.000; rss: 75MB  loop checking
time: 0.000; rss: 75MB  static item recursion checking
time: 0.001; rss: 75MB  compute_incremental_hashes_map
time: 0.000; rss: 75MB  load_dep_graph
time: 0.000; rss: 75MB  type collecting
time: 0.000; rss: 75MB  variance inference
time: 0.000; rss: 75MB  impl wf inference
time: 0.045; rss: 78MB  coherence checking
time: 0.001; rss: 78MB  wf checking
time: 0.000; rss: 78MB  item-types checking
time: 0.030; rss: 79MB  item-bodies checking
time: 0.000; rss: 79MB  drop-impl checking
time: 0.001; rss: 79MB  const checking
time: 0.000; rss: 79MB  privacy checking
time: 0.000; rss: 79MB  stability index
time: 0.000; rss: 79MB  intrinsic checking
time: 0.000; rss: 79MB  effect checking
time: 0.000; rss: 79MB  match checking
time: 0.000; rss: 79MB  liveness checking
time: 0.000; rss: 79MB  rvalue checking
time: 0.002; rss: 80MB  MIR dump
  time: 0.000; rss: 80MB        SimplifyCfg
  time: 0.001; rss: 80MB        QualifyAndPromoteConstants
  time: 0.000; rss: 80MB        TypeckMir
  time: 0.000; rss: 80MB        SimplifyBranches
  time: 0.000; rss: 80MB        SimplifyCfg
time: 0.002; rss: 80MB  MIR cleanup and validation
time: 0.002; rss: 80MB  borrow checking
time: 0.000; rss: 80MB  reachability checking
time: 0.000; rss: 80MB  death checking
time: 0.001; rss: 80MB  stability checking
time: 0.000; rss: 80MB  unused lib feature checking
time: 0.001; rss: 80MB  lint checking
time: 0.003; rss: 80MB  resolving dependency formats
  time: 0.000; rss: 80MB        NoLandingPads
  time: 0.000; rss: 80MB        SimplifyCfg
  time: 0.000; rss: 80MB        EraseRegions
  time: 0.000; rss: 80MB        AddCallGuards
  time: 0.001; rss: 80MB        ElaborateDrops
  time: 0.000; rss: 80MB        NoLandingPads
  time: 0.000; rss: 80MB        SimplifyCfg
  time: 0.000; rss: 80MB        InstCombine
  time: 0.000; rss: 80MB        Deaggregator
  time: 0.000; rss: 80MB        CopyPropagation
  time: 0.000; rss: 80MB        SimplifyLocals
  time: 0.000; rss: 80MB        AddCallGuards
  time: 0.000; rss: 80MB        PreTrans
time: 0.005; rss: 80MB  MIR optimisations
  time: 0.000; rss: 81MB        write metadata
  time: 0.026; rss: 83MB        translation item collection
  time: 0.002; rss: 83MB        codegen unit partitioning
  time: 0.003; rss: 87MB        internalize symbols
time: 0.118; rss: 87MB  translation
time: 0.000; rss: 87MB  assert dep graph
time: 0.000; rss: 88MB  serialize dep graph
  time: 0.009; rss: 84MB        llvm function passes [0]
  time: 0.001; rss: 85MB        llvm module passes [0]
  time: 0.165; rss: 90MB        codegen passes [0]
  time: 0.002; rss: 89MB        codegen passes [0]
time: 0.179; rss: 89MB  LLVM passes
time: 0.000; rss: 89MB  serialize work products
  time: 0.382; rss: 87MB        running linker
time: 0.386; rss: 87MB  linking
    Finished debug [unoptimized + debuginfo] target(s) in 1.1 secs

real    0m1.236s
user    0m0.000s
sys     0m0.000s
medium
   Compiling testingtesting v0.1.0 (file:///C:/Users/steve/tmp/testingtesting)
time: 0.000; rss: 25MB  parsing
time: 0.000; rss: 25MB  recursion limit
time: 0.000; rss: 25MB  crate injection
time: 0.006; rss: 30MB  plugin loading
time: 0.000; rss: 30MB  plugin registration
time: 0.184; rss: 78MB  expansion
time: 0.000; rss: 78MB  maybe building test harness
time: 0.000; rss: 78MB  maybe creating a macro crate
time: 0.000; rss: 78MB  checking for inline asm in case the target doesn't support it
time: 0.001; rss: 78MB  complete gated feature checking
time: 0.002; rss: 78MB  early lint checks
time: 0.000; rss: 78MB  AST validation
time: 0.012; rss: 79MB  name resolution
time: 0.021; rss: 82MB  lowering ast -> hir
time: 0.001; rss: 83MB  indexing hir
time: 0.000; rss: 83MB  attribute checking
time: 0.001; rss: 80MB  language item collection
time: 0.001; rss: 80MB  lifetime resolution
time: 0.000; rss: 80MB  looking for entry point
time: 0.000; rss: 80MB  looking for plugin registrar
time: 0.006; rss: 83MB  region resolution
time: 0.001; rss: 83MB  loop checking
time: 0.001; rss: 83MB  static item recursion checking
time: 0.057; rss: 83MB  compute_incremental_hashes_map
time: 0.000; rss: 83MB  load_dep_graph
time: 0.001; rss: 83MB  type collecting
time: 0.000; rss: 83MB  variance inference
time: 0.000; rss: 83MB  impl wf inference
time: 0.045; rss: 85MB  coherence checking
time: 0.002; rss: 85MB  wf checking
time: 0.001; rss: 85MB  item-types checking
time: 1.665; rss: 90MB  item-bodies checking
time: 0.000; rss: 90MB  drop-impl checking
time: 0.030; rss: 91MB  const checking
time: 0.002; rss: 91MB  privacy checking
time: 0.000; rss: 91MB  stability index
time: 0.001; rss: 91MB  intrinsic checking
time: 0.001; rss: 91MB  effect checking
time: 0.033; rss: 91MB  match checking
time: 0.212; rss: 92MB  liveness checking
time: 0.030; rss: 92MB  rvalue checking
time: 0.081; rss: 113MB MIR dump
  time: 0.015; rss: 113MB       SimplifyCfg
  time: 0.014; rss: 114MB       QualifyAndPromoteConstants
  time: 0.020; rss: 115MB       TypeckMir
  time: 0.000; rss: 115MB       SimplifyBranches
  time: 0.004; rss: 115MB       SimplifyCfg
time: 0.056; rss: 115MB MIR cleanup and validation
time: 0.355; rss: 114MB borrow checking
time: 0.000; rss: 114MB reachability checking
time: 0.004; rss: 114MB death checking
time: 0.003; rss: 114MB stability checking
time: 0.000; rss: 114MB unused lib feature checking
time: 0.024; rss: 115MB lint checking
time: 0.005; rss: 115MB resolving dependency formats
  time: 0.000; rss: 115MB       NoLandingPads
  time: 0.004; rss: 115MB       SimplifyCfg
  time: 0.015; rss: 116MB       EraseRegions
  time: 0.001; rss: 116MB       AddCallGuards
  time: 0.153; rss: 116MB       ElaborateDrops
  time: 0.000; rss: 116MB       NoLandingPads
  time: 0.004; rss: 116MB       SimplifyCfg
  time: 0.004; rss: 116MB       InstCombine
  time: 0.001; rss: 116MB       Deaggregator
  time: 0.000; rss: 116MB       CopyPropagation
  time: 0.006; rss: 116MB       SimplifyLocals
  time: 0.001; rss: 116MB       AddCallGuards
  time: 0.000; rss: 116MB       PreTrans
time: 0.194; rss: 116MB MIR optimisations
  time: 0.000; rss: 117MB       write metadata
  time: 0.073; rss: 117MB       translation item collection
  time: 0.002; rss: 117MB       codegen unit partitioning
  time: 0.002; rss: 129MB       internalize symbols
time: 0.370; rss: 129MB translation
time: 0.000; rss: 129MB assert dep graph
time: 0.000; rss: 129MB serialize dep graph
  time: 0.046; rss: 93MB        llvm function passes [0]
  time: 0.011; rss: 93MB        llvm module passes [0]
  time: 0.563; rss: 99MB        codegen passes [0]
  time: 0.002; rss: 98MB        codegen passes [0]
time: 0.624; rss: 98MB  LLVM passes
time: 0.000; rss: 98MB  serialize work products
  time: 0.405; rss: 90MB        running linker
time: 0.410; rss: 90MB  linking
    Finished debug [unoptimized + debuginfo] target(s) in 4.59 secs

real    0m4.807s
user    0m0.000s
sys     0m0.015s
long
   Compiling testingtesting v0.1.0 (file:///C:/Users/steve/tmp/testingtesting)
time: 0.000; rss: 24MB  parsing
time: 0.000; rss: 24MB  recursion limit
time: 0.000; rss: 24MB  crate injection
time: 0.006; rss: 28MB  plugin loading
time: 0.000; rss: 29MB  plugin registration
time: 0.313; rss: 80MB  expansion
time: 0.000; rss: 80MB  maybe building test harness
time: 0.001; rss: 80MB  maybe creating a macro crate
time: 0.000; rss: 80MB  checking for inline asm in case the target doesn't support it
time: 0.002; rss: 80MB  complete gated feature checking
time: 0.005; rss: 80MB  early lint checks
time: 0.003; rss: 80MB  AST validation
time: 0.026; rss: 81MB  name resolution
time: 0.087; rss: 88MB  lowering ast -> hir
time: 0.004; rss: 88MB  indexing hir
time: 0.003; rss: 88MB  attribute checking
time: 0.002; rss: 85MB  language item collection
time: 0.002; rss: 85MB  lifetime resolution
time: 0.000; rss: 85MB  looking for entry point
time: 0.000; rss: 85MB  looking for plugin registrar
time: 0.018; rss: 89MB  region resolution
time: 0.003; rss: 89MB  loop checking
time: 0.001; rss: 89MB  static item recursion checking
time: 0.119; rss: 89MB  compute_incremental_hashes_map
time: 0.000; rss: 89MB  load_dep_graph
time: 0.001; rss: 89MB  type collecting
time: 0.000; rss: 89MB  variance inference
time: 0.000; rss: 89MB  impl wf inference
time: 0.046; rss: 90MB  coherence checking
time: 0.002; rss: 91MB  wf checking
time: 0.002; rss: 91MB  item-types checking
time: 5.346; rss: 100MB item-bodies checking
time: 0.000; rss: 100MB drop-impl checking
time: 0.063; rss: 102MB const checking
time: 0.005; rss: 102MB privacy checking
time: 0.001; rss: 102MB stability index
time: 0.002; rss: 102MB intrinsic checking
time: 0.003; rss: 102MB effect checking
time: 0.114; rss: 102MB match checking
time: 0.825; rss: 103MB liveness checking
time: 0.060; rss: 103MB rvalue checking
time: 0.161; rss: 144MB MIR dump
  time: 0.028; rss: 145MB       SimplifyCfg
  time: 0.024; rss: 147MB       QualifyAndPromoteConstants
  time: 0.037; rss: 148MB       TypeckMir
  time: 0.001; rss: 148MB       SimplifyBranches
  time: 0.008; rss: 148MB       SimplifyCfg
time: 0.099; rss: 148MB MIR cleanup and validation
time: 1.172; rss: 147MB borrow checking
time: 0.000; rss: 147MB reachability checking
time: 0.011; rss: 147MB death checking
time: 0.010; rss: 148MB stability checking
time: 0.000; rss: 148MB unused lib feature checking
time: 0.053; rss: 148MB lint checking
time: 0.003; rss: 148MB resolving dependency formats
  time: 0.000; rss: 148MB       NoLandingPads
  time: 0.008; rss: 148MB       SimplifyCfg
  time: 0.027; rss: 149MB       EraseRegions
  time: 0.002; rss: 149MB       AddCallGuards
  time: 0.532; rss: 150MB       ElaborateDrops
  time: 0.000; rss: 150MB       NoLandingPads
  time: 0.008; rss: 150MB       SimplifyCfg
  time: 0.010; rss: 150MB       InstCombine
  time: 0.001; rss: 150MB       Deaggregator
  time: 0.000; rss: 150MB       CopyPropagation
  time: 0.013; rss: 150MB       SimplifyLocals
  time: 0.002; rss: 150MB       AddCallGuards
  time: 0.000; rss: 150MB       PreTrans
time: 0.607; rss: 150MB MIR optimisations
  time: 0.000; rss: 150MB       write metadata
  time: 0.099; rss: 151MB       translation item collection
  time: 0.002; rss: 151MB       codegen unit partitioning
  time: 0.002; rss: 168MB       internalize symbols
time: 0.548; rss: 168MB translation
time: 0.000; rss: 168MB assert dep graph
time: 0.000; rss: 168MB serialize dep graph
  time: 0.092; rss: 99MB        llvm function passes [0]
  time: 0.018; rss: 100MB       llvm module passes [0]
  time: 0.953; rss: 107MB       codegen passes [0]
  time: 0.002; rss: 106MB       codegen passes [0]
time: 1.067; rss: 106MB LLVM passes
time: 0.000; rss: 106MB serialize work products
  time: 0.420; rss: 91MB        running linker
time: 0.425; rss: 91MB  linking
    Finished debug [unoptimized + debuginfo] target(s) in 11.39 secs

real    0m11.610s
user    0m0.000s
sys     0m0.000s
