
time: 0.329; rss: 77MB  parsing
time: 0.000; rss: 77MB  recursion limit
time: 0.000; rss: 77MB  crate injection
time: 0.000; rss: 77MB  plugin loading
time: 0.000; rss: 77MB  plugin registration
time: 2.079; rss: 135MB expansion
time: 0.000; rss: 135MB maybe building test harness
time: 0.006; rss: 135MB maybe creating a macro crate
time: 0.000; rss: 135MB checking for inline asm in case the target doesn't support it
time: 0.014; rss: 135MB complete gated feature checking
time: 0.024; rss: 135MB early lint checks
time: 0.007; rss: 135MB AST validation
time: 0.179; rss: 150MB name resolution
time: 0.282; rss: 183MB lowering ast -> hir
time: 0.016; rss: 196MB indexing hir
time: 0.006; rss: 196MB attribute checking
time: 0.006; rss: 145MB language item collection
time: 0.018; rss: 146MB lifetime resolution
time: 0.000; rss: 146MB looking for entry point
time: 0.000; rss: 146MB looking for plugin registrar
time: 0.052; rss: 155MB region resolution
time: 0.005; rss: 155MB loop checking
time: 0.005; rss: 155MB static item recursion checking
time: 0.179; rss: 157MB compute_incremental_hashes_map
time: 0.000; rss: 157MB load_dep_graph
time: 0.438; rss: 167MB type collecting
time: 0.003; rss: 167MB variance inference
time: 2.307; rss: 169MB coherence checking
time: 3.236; rss: 176MB wf checking
time: 2.069; rss: 182MB item-types checking
time: 11.483; rss: 209MB    item-bodies checking
time: 0.000; rss: 209MB drop-impl checking
time: 0.558; rss: 210MB const checking
time: 0.077; rss: 210MB privacy checking
time: 0.011; rss: 210MB stability index
time: 0.122; rss: 210MB intrinsic checking
time: 0.013; rss: 210MB effect checking
time: 0.373; rss: 210MB match checking
time: 0.102; rss: 211MB liveness checking
time: 0.537; rss: 211MB rvalue checking
time: 1.173; rss: 286MB MIR dump
  time: 0.248; rss: 287MB   SimplifyCfg
  time: 0.324; rss: 288MB   QualifyAndPromoteConstants
  time: 0.329; rss: 288MB   TypeckMir
  time: 0.006; rss: 288MB   SimplifyBranches
  time: 0.169; rss: 288MB   SimplifyCfg
time: 1.077; rss: 288MB MIR passes
time: 1.901; rss: 289MB borrow checking
time: 0.127; rss: 290MB reachability checking
time: 0.049; rss: 291MB death checking
time: 0.041; rss: 291MB stability checking
time: 0.000; rss: 291MB unused lib feature checking
time: 1.482; rss: 291MB lint checking
time: 0.000; rss: 291MB resolving dependency formats
  time: 0.006; rss: 291MB   NoLandingPads
  time: 0.163; rss: 291MB   SimplifyCfg
  time: 0.069; rss: 294MB   EraseRegions
  time: 0.064; rss: 294MB   AddCallGuards
  time: 0.543; rss: 298MB   ElaborateDrops
  time: 0.006; rss: 298MB   NoLandingPads
  time: 0.189; rss: 299MB   SimplifyCfg
  time: 0.035; rss: 299MB   InstCombine
  time: 0.116; rss: 299MB   Deaggregator
  time: 0.006; rss: 299MB   CopyPropagation
  time: 0.059; rss: 299MB   AddCallGuards
  time: 0.022; rss: 299MB   PreTrans
time: 1.280; rss: 299MB Prepare MIR codegen passes
  time: 0.133; rss: 323MB   write metadata
  time: 0.138; rss: 324MB   translation item collection
  time: 0.205; rss: 325MB   codegen unit partitioning
  time: 0.003; rss: 326MB   internalize symbols
time: 1.613; rss: 254MB translation
time: 0.002; rss: 254MB assert dep graph
time: 0.000; rss: 254MB serialize dep graph
  time: 0.061; rss: 97MB    llvm function passes [0]
  time: 0.023; rss: 98MB    llvm module passes [0]
  time: 1.171; rss: 106MB   codegen passes [0]
  time: 0.001; rss: 106MB   codegen passes [0]
time: 1.306; rss: 106MB LLVM passes
time: 0.000; rss: 106MB serialize work products
time: 0.025; rss: 93MB  linking
    Command being timed: "/home/mark/tmp/with-changes/stage2/bin/rustc -Ztime-passes src/libcore/lib.rs"
    User time (seconds): 35.40
    System time (seconds): 0.18
    Percent of CPU this job got: 99%
    Elapsed (wall clock) time (h:mm:ss or m:ss): 0:35.59
    Average shared text size (kbytes): 0
    Average unshared data size (kbytes): 0
    Average stack size (kbytes): 0
    Average total size (kbytes): 0
    Maximum resident set size (kbytes): 318360
    Average resident set size (kbytes): 0
    Major (requiring I/O) page faults: 0
    Minor (reclaiming a frame) page faults: 117601
    Voluntary context switches: 2
    Involuntary context switches: 47
    Swaps: 0
    File system inputs: 0
    File system outputs: 36016
    Socket messages sent: 0
    Socket messages received: 0
    Signals delivered: 0
    Page size (bytes): 4096
    Exit status: 0
