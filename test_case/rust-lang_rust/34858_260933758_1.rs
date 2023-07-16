
$ cargo rustc  -- -Ztime-passes -A warnings
   Compiling test_bug v0.1.0 (file://test_bug)
warning: the option `Z` is unstable and should only be used on the nightly compiler, but it is currently accepted for backwards compatibility; this will soon change, see issue #31847 for more details

time: 0.000; rss: 45MB  parsing
time: 0.000; rss: 45MB  recursion limit
time: 0.000; rss: 45MB  crate injection
time: 0.000; rss: 45MB  plugin loading
time: 0.000; rss: 45MB  plugin registration
time: 0.019; rss: 65MB  expansion
time: 0.000; rss: 65MB  maybe building test harness
time: 0.000; rss: 65MB  maybe creating a macro crate
time: 0.000; rss: 65MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 65MB  complete gated feature checking
time: 0.000; rss: 65MB  collecting defs
time: 0.000; rss: 65MB  early lint checks
time: 0.000; rss: 65MB  AST validation
time: 0.003; rss: 65MB  name resolution
time: 0.000; rss: 65MB  lowering ast -> hir
time: 0.000; rss: 65MB  indexing hir
time: 0.000; rss: 65MB  attribute checking
time: 0.000; rss: 65MB  language item collection
time: 0.000; rss: 65MB  lifetime resolution
time: 0.000; rss: 65MB  looking for entry point
time: 0.000; rss: 65MB  looking for plugin registrar
time: 0.000; rss: 65MB  region resolution
time: 0.000; rss: 65MB  loop checking
time: 0.000; rss: 65MB  static item recursion checking
time: 0.000; rss: 69MB  compute_incremental_hashes_map
time: 0.000; rss: 69MB  load_dep_graph
time: 0.000; rss: 69MB  type collecting
time: 0.000; rss: 69MB  variance inference
time: 0.010; rss: 69MB  coherence checking
time: 0.000; rss: 69MB  wf checking
time: 0.000; rss: 69MB  item-types checking
time: 0.001; rss: 69MB  item-bodies checking
time: 0.000; rss: 69MB  drop-impl checking
time: 0.000; rss: 69MB  const checking
time: 0.000; rss: 69MB  privacy checking
time: 0.000; rss: 69MB  stability index
time: 0.000; rss: 69MB  intrinsic checking
time: 0.000; rss: 69MB  effect checking
time: 0.000; rss: 69MB  match checking
time: 0.000; rss: 69MB  liveness checking
time: 0.000; rss: 69MB  rvalue checking
time: 0.000; rss: 69MB  MIR dump
  time: 0.000; rss: 69MB        SimplifyCfg
  time: 0.000; rss: 69MB        QualifyAndPromoteConstants
  time: 0.000; rss: 69MB        TypeckMir
  time: 0.000; rss: 69MB        SimplifyBranches
  time: 0.000; rss: 69MB        SimplifyCfg
time: 0.000; rss: 69MB  MIR passes
time: 0.000; rss: 69MB  borrow checking
time: 0.000; rss: 69MB  reachability checking
time: 0.000; rss: 69MB  death checking
time: 0.000; rss: 69MB  stability checking
time: 0.000; rss: 69MB  unused lib feature checking
time: 0.000; rss: 69MB  lint checking
time: 0.001; rss: 69MB  resolving dependency formats
  time: 0.000; rss: 69MB        NoLandingPads
  time: 0.000; rss: 69MB        SimplifyCfg
  time: 0.000; rss: 69MB        EraseRegions
  time: 0.000; rss: 69MB        AddCallGuards
  time: 0.000; rss: 69MB        ElaborateDrops
  time: 0.000; rss: 69MB        NoLandingPads
  time: 0.000; rss: 69MB        SimplifyCfg
  time: 0.000; rss: 69MB        InstCombine
  time: 0.000; rss: 69MB        Deaggregator
  time: 0.000; rss: 69MB        CopyPropagation
  time: 0.000; rss: 69MB        AddCallGuards
  time: 0.000; rss: 69MB        PreTrans
time: 0.000; rss: 69MB  Prepare MIR codegen passes
  time: 0.000; rss: 69MB        write metadata

