
[chrivers@helios]~/git/artemis/oxide #cargo rustc --lib -- -Ztime-passes
   Compiling oxide v0.1.0 (file:///home/chrivers/git/artemis/oxide)
time: 0.014; rss: 53MB  parsing
time: 0.000; rss: 53MB  recursion limit
time: 0.000; rss: 53MB  crate injection
time: 0.000; rss: 53MB  plugin loading
time: 0.000; rss: 53MB  plugin registration
time: 0.587; rss: 136MB expansion
time: 0.000; rss: 136MB maybe building test harness
time: 0.005; rss: 136MB maybe creating a macro crate
time: 0.000; rss: 136MB checking for inline asm in case the target doesn't support it
time: 0.009; rss: 136MB complete gated feature checking
time: 0.032; rss: 138MB collecting defs
time: 0.017; rss: 142MB external crate/lib resolution
time: 0.022; rss: 142MB early lint checks
time: 0.007; rss: 142MB AST validation
time: 0.074; rss: 159MB name resolution
time: 0.042; rss: 187MB lowering ast -> hir
time: 0.009; rss: 192MB indexing hir
time: 0.006; rss: 192MB attribute checking
time: 0.005; rss: 147MB language item collection
time: 0.008; rss: 147MB lifetime resolution
time: 0.000; rss: 147MB looking for entry point
time: 0.000; rss: 147MB looking for plugin registrar
time: 0.037; rss: 157MB region resolution
time: 0.005; rss: 157MB loop checking
time: 0.005; rss: 157MB static item recursion checking
time: 0.221; rss: 159MB compute_incremental_hashes_map
time: 0.000; rss: 159MB load_dep_graph
time: 0.020; rss: 161MB type collecting
time: 0.001; rss: 161MB variance inference
time: 0.040; rss: 167MB coherence checking
time: 0.044; rss: 168MB wf checking
time: 0.144; rss: 171MB item-types checking
time: 2.333; rss: 204MB item-bodies checking
time: 0.000; rss: 204MB drop-impl checking
time: 0.157; rss: 206MB const checking
time: 0.039; rss: 206MB privacy checking
time: 0.006; rss: 206MB stability index
time: 0.018; rss: 206MB intrinsic checking
time: 0.017; rss: 206MB effect checking
time: 0.038; rss: 206MB match checking
time: 0.149; rss: 205MB liveness checking
time: 0.116; rss: 205MB rvalue checking
time: 0.277; rss: 320MB MIR dump
  time: 0.042; rss: 331MB       SimplifyCfg
  time: 0.055; rss: 331MB       QualifyAndPromoteConstants
  time: 0.110; rss: 332MB       TypeckMir
  time: 0.003; rss: 332MB       SimplifyBranches
  time: 0.026; rss: 332MB       SimplifyCfg
time: 0.235; rss: 332MB MIR passes
time: 1.213; rss: 349MB borrow checking
time: 0.001; rss: 349MB reachability checking
time: 0.033; rss: 349MB death checking
time: 0.030; rss: 349MB stability checking
time: 0.000; rss: 349MB unused lib feature checking
warning: this feature has been stable since 1.13.0. Attribute no longer needed, #[warn(stable_features)] on by default
 --> src/lib.rs:2:12
  |
2 | #![feature(type_macros)]
  |            ^^^^^^^^^^^

time: 0.171; rss: 349MB lint checking
time: 0.000; rss: 349MB resolving dependency formats
  time: 0.002; rss: 349MB       NoLandingPads
  time: 0.017; rss: 349MB       SimplifyCfg
  time: 0.061; rss: 349MB       EraseRegions
  time: 0.008; rss: 349MB       AddCallGuards
  time: 1.188; rss: 377MB       ElaborateDrops
  time: 0.002; rss: 377MB       NoLandingPads
  time: 0.034; rss: 348MB       SimplifyCfg
  time: 0.023; rss: 348MB       InstCombine
  time: 0.007; rss: 348MB       Deaggregator
  time: 6.139; rss: 352MB       CopyPropagation
  time: 0.007; rss: 352MB       AddCallGuards
  time: 0.002; rss: 352MB       PreTrans
time: 7.490; rss: 352MB Prepare MIR codegen passes
  time: 0.049; rss: 353MB       write metadata
  time: 0.139; rss: 356MB       translation item collection
  time: 0.015; rss: 359MB       codegen unit partitioning
  time: 0.007; rss: 417MB       internalize symbols
time: 1.690; rss: 318MB translation
time: 0.000; rss: 318MB assert dep graph
time: 0.000; rss: 318MB serialize dep graph
  time: 0.196; rss: 185MB       llvm function passes [0]
  time: 0.095; rss: 189MB       llvm module passes [0]
  time: 3.877; rss: 194MB       codegen passes [0]
  time: 0.001; rss: 194MB       codegen passes [0]
time: 4.401; rss: 194MB LLVM passes
time: 0.000; rss: 194MB serialize work products
time: 0.239; rss: 112MB linking
    Finished debug [unoptimized + debuginfo] target(s) in 20.18 secs
