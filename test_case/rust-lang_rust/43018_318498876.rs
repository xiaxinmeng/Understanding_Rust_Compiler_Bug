console
$ env CARGO_INCREMENTAL= RUSTC_BOOTSTRAP=1 cargo +stable rustc -- -Z time-passes
   Compiling rust-issue-43018 v0.1.0 (file:///home/jon/tmp/rust-issue-43018)
time: 0.000; rss: 44MB  parsing
time: 0.000; rss: 44MB  recursion limit
time: 0.000; rss: 44MB  crate injection
time: 0.000; rss: 44MB  plugin loading
time: 0.000; rss: 44MB  plugin registration
time: 0.028; rss: 73MB  expansion
time: 0.000; rss: 73MB  maybe building test harness
time: 0.000; rss: 73MB  maybe creating a macro crate
time: 0.000; rss: 73MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 73MB  early lint checks
time: 0.000; rss: 73MB  AST validation
time: 0.002; rss: 77MB  name resolution
time: 0.000; rss: 77MB  complete gated feature checking
time: 0.000; rss: 77MB  lowering ast -> hir
time: 0.000; rss: 77MB  indexing hir
time: 0.000; rss: 77MB  attribute checking
time: 0.000; rss: 77MB  language item collection
time: 0.000; rss: 77MB  lifetime resolution
time: 0.000; rss: 77MB  looking for entry point
time: 0.000; rss: 77MB  looking for plugin registrar
time: 0.000; rss: 77MB  loop checking
time: 0.000; rss: 77MB  static item recursion checking
time: 0.000; rss: 77MB  compute_incremental_hashes_map
time: 0.000; rss: 77MB  load_dep_graph
time: 0.000; rss: 77MB  stability index
time: 0.001; rss: 77MB  stability checking
time: 0.000; rss: 77MB  type collecting
time: 0.000; rss: 77MB  impl wf inference
time: 0.000; rss: 77MB  coherence checking
time: 0.000; rss: 77MB  variance testing
time: 0.000; rss: 77MB  wf checking
time: 0.000; rss: 77MB  item-types checking
time: 0.030; rss: 99MB  item-bodies checking
time: 0.001; rss: 99MB  const checking
time: 0.000; rss: 99MB  privacy checking
time: 0.000; rss: 99MB  intrinsic checking
time: 0.000; rss: 99MB  effect checking
time: 0.000; rss: 99MB  match checking
time: 0.000; rss: 99MB  liveness checking
time: 1.107; rss: 99MB  borrow checking
time: 0.000; rss: 99MB  reachability checking
time: 0.000; rss: 99MB  death checking
time: 0.000; rss: 99MB  unused lib feature checking
time: 0.000; rss: 99MB  lint checking
time: 0.000; rss: 99MB  resolving dependency formats
  time: 0.000; rss: 99MB        write metadata
  time: 26.469; rss: 107MB      translation item collection
  time: 0.001; rss: 107MB       codegen unit partitioning
  time: 0.000; rss: 122MB       internalize symbols
time: 40.250; rss: 122MB        translation
time: 0.000; rss: 122MB assert dep graph
time: 0.000; rss: 122MB serialize dep graph
  time: 0.010; rss: 124MB       llvm function passes [0]
  time: 0.004; rss: 124MB       llvm module passes [0]
  time: 0.215; rss: 129MB       codegen passes [0]
  time: 0.000; rss: 129MB       codegen passes [0]
time: 0.231; rss: 129MB LLVM passes
time: 0.000; rss: 129MB serialize work products
  time: 0.099; rss: 125MB       running linker
time: 0.100; rss: 125MB linking
    Finished dev [unoptimized + debuginfo] target(s) in 41.83 secs
$ env CARGO_INCREMENTAL= RUSTC_BOOTSTRAP=1 cargo +1.18.0 rustc -- -Z time-passes
   Compiling futures v0.1.14
   Compiling rust-issue-43018 v0.1.0 (file:///home/jon/tmp/rust-issue-43018)
time: 0.000; rss: 36MB  parsing
time: 0.000; rss: 40MB  recursion limit
time: 0.000; rss: 40MB  crate injection
time: 0.000; rss: 40MB  plugin loading
time: 0.000; rss: 40MB  plugin registration
time: 0.027; rss: 73MB  expansion
time: 0.000; rss: 73MB  maybe building test harness
time: 0.000; rss: 73MB  maybe creating a macro crate
time: 0.000; rss: 73MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 73MB  early lint checks
time: 0.000; rss: 73MB  AST validation
time: 0.001; rss: 73MB  name resolution
time: 0.000; rss: 73MB  complete gated feature checking
time: 0.000; rss: 73MB  lowering ast -> hir
time: 0.000; rss: 73MB  indexing hir
time: 0.000; rss: 73MB  attribute checking
time: 0.000; rss: 73MB  language item collection
time: 0.000; rss: 73MB  lifetime resolution
time: 0.000; rss: 73MB  looking for entry point
time: 0.000; rss: 73MB  looking for plugin registrar
time: 0.000; rss: 73MB  region resolution
time: 0.000; rss: 73MB  loop checking
time: 0.000; rss: 73MB  static item recursion checking
time: 0.000; rss: 73MB  compute_incremental_hashes_map
time: 0.000; rss: 73MB  load_dep_graph
time: 0.000; rss: 73MB  stability index
time: 0.000; rss: 73MB  stability checking
time: 0.000; rss: 77MB  type collecting
time: 0.000; rss: 77MB  variance inference
time: 0.000; rss: 77MB  impl wf inference
time: 0.000; rss: 77MB  coherence checking
time: 0.000; rss: 77MB  wf checking
time: 0.000; rss: 77MB  item-types checking
time: 0.031; rss: 95MB  item-bodies checking
time: 0.000; rss: 95MB  const checking
time: 0.000; rss: 95MB  privacy checking
time: 0.000; rss: 95MB  intrinsic checking
time: 0.000; rss: 95MB  effect checking
time: 0.000; rss: 95MB  match checking
time: 0.000; rss: 95MB  liveness checking
time: 0.001; rss: 95MB  MIR dump
  time: 0.000; rss: 95MB        SimplifyCfg
  time: 0.000; rss: 95MB        TypeckMir
  time: 0.000; rss: 95MB        QualifyAndPromoteConstants
  time: 0.000; rss: 95MB        SimplifyBranches
  time: 0.000; rss: 95MB        SimplifyCfg
time: 0.001; rss: 95MB  MIR cleanup and validation
time: 0.000; rss: 95MB  borrow checking
time: 0.000; rss: 95MB  reachability checking
time: 0.000; rss: 95MB  death checking
time: 0.000; rss: 95MB  unused lib feature checking
time: 0.000; rss: 95MB  lint checking
time: 0.000; rss: 95MB  resolving dependency formats
  time: 0.000; rss: 95MB        NoLandingPads
  time: 0.000; rss: 95MB        SimplifyCfg
  time: 0.000; rss: 95MB        EraseRegions
  time: 0.000; rss: 95MB        AddCallGuards
  time: 0.000; rss: 95MB        ElaborateDrops
  time: 0.000; rss: 95MB        NoLandingPads
  time: 0.000; rss: 95MB        SimplifyCfg
  time: 0.000; rss: 95MB        Inline
  time: 0.000; rss: 95MB        InstCombine
  time: 0.000; rss: 95MB        Deaggregator
  time: 0.000; rss: 95MB        CopyPropagation
  time: 0.000; rss: 95MB        SimplifyLocals
  time: 0.000; rss: 95MB        AddCallGuards
  time: 0.000; rss: 95MB        PreTrans
time: 0.001; rss: 95MB  MIR optimisations
  time: 0.000; rss: 95MB        write metadata
  time: 22.824; rss: 101MB      translation item collection
  time: 0.001; rss: 101MB       codegen unit partitioning
  time: 0.000; rss: 120MB       internalize symbols
time: 37.191; rss: 120MB        translation
time: 0.000; rss: 120MB assert dep graph
time: 0.000; rss: 120MB serialize dep graph
  time: 0.009; rss: 120MB       llvm function passes [0]
  time: 0.003; rss: 120MB       llvm module passes [0]
  time: 0.200; rss: 126MB       codegen passes [0]
  time: 0.000; rss: 126MB       codegen passes [0]
time: 0.214; rss: 125MB LLVM passes
time: 0.000; rss: 125MB serialize work products
  time: 0.109; rss: 118MB       running linker
time: 0.110; rss: 118MB linking
    Finished dev [unoptimized + debuginfo] target(s) in 39.68 secs
