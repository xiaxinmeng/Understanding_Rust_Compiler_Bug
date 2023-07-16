
time: 0.336; rss: 75MB  parsing
time: 0.000; rss: 75MB  recursion limit
time: 0.000; rss: 75MB  crate injection
time: 0.000; rss: 75MB  plugin loading
time: 0.000; rss: 75MB  plugin registration
time: 2.077; rss: 134MB expansion
time: 0.000; rss: 134MB maybe building test harness
time: 0.006; rss: 134MB maybe creating a macro crate
time: 0.000; rss: 134MB checking for inline asm in case the target doesn't support it
time: 0.014; rss: 134MB complete gated feature checking
time: 0.023; rss: 134MB early lint checks
time: 0.007; rss: 134MB AST validation
time: 0.180; rss: 149MB name resolution
time: 0.283; rss: 182MB lowering ast -> hir
time: 0.016; rss: 195MB indexing hir
time: 0.006; rss: 195MB attribute checking
time: 0.006; rss: 144MB language item collection
time: 0.018; rss: 145MB lifetime resolution
time: 0.000; rss: 145MB looking for entry point
time: 0.000; rss: 145MB looking for plugin registrar
time: 0.053; rss: 153MB region resolution
time: 0.005; rss: 153MB loop checking
time: 0.005; rss: 153MB static item recursion checking
time: 0.199; rss: 155MB compute_incremental_hashes_map
time: 0.000; rss: 155MB load_dep_graph
time: 0.435; rss: 167MB type collecting
time: 0.003; rss: 167MB variance inference
time: 2.311; rss: 168MB coherence checking
time: 3.240; rss: 175MB wf checking
time: 2.080; rss: 181MB item-types checking
time: 11.483; rss: 208MB    item-bodies checking
time: 0.000; rss: 208MB drop-impl checking
time: 0.561; rss: 209MB const checking
time: 0.079; rss: 209MB privacy checking
time: 0.011; rss: 209MB stability index
time: 0.123; rss: 209MB intrinsic checking
time: 0.013; rss: 209MB effect checking
time: 0.372; rss: 209MB match checking
time: 0.101; rss: 209MB liveness checking
time: 0.538; rss: 209MB rvalue checking
time: 1.174; rss: 285MB MIR dump
  time: 0.248; rss: 287MB   SimplifyCfg
  time: 0.326; rss: 287MB   QualifyAndPromoteConstants
  time: 0.331; rss: 287MB   TypeckMir
  time: 0.006; rss: 287MB   SimplifyBranches
  time: 0.168; rss: 287MB   SimplifyCfg
time: 1.080; rss: 287MB MIR passes
time: 1.910; rss: 288MB borrow checking
time: 0.127; rss: 289MB reachability checking
time: 0.050; rss: 290MB death checking
time: 0.041; rss: 290MB stability checking
time: 0.000; rss: 290MB unused lib feature checking
time: 1.467; rss: 291MB lint checking
time: 0.000; rss: 291MB resolving dependency formats
  time: 0.006; rss: 291MB   NoLandingPads
  time: 0.162; rss: 291MB   SimplifyCfg
  time: 0.070; rss: 293MB   EraseRegions
  time: 0.065; rss: 293MB   AddCallGuards
  time: 0.541; rss: 297MB   ElaborateDrops
  time: 0.007; rss: 297MB   NoLandingPads
  time: 0.189; rss: 297MB   SimplifyCfg
  time: 0.034; rss: 297MB   InstCombine
  time: 0.117; rss: 297MB   Deaggregator
  time: 0.007; rss: 297MB   CopyPropagation
  time: 0.060; rss: 297MB   AddCallGuards
  time: 0.022; rss: 297MB   PreTrans
time: 1.281; rss: 297MB Prepare MIR codegen passes
  time: 0.135; rss: 322MB   write metadata
  time: 0.139; rss: 324MB   translation item collection
  time: 0.209; rss: 324MB   codegen unit partitioning
  time: 0.003; rss: 325MB   internalize symbols
time: 1.636; rss: 254MB translation
time: 0.002; rss: 254MB assert dep graph
time: 0.000; rss: 254MB serialize dep graph
  time: 0.061; rss: 96MB    llvm function passes [0]
  time: 0.023; rss: 97MB    llvm module passes [0]
  time: 1.173; rss: 105MB   codegen passes [0]
  time: 0.001; rss: 105MB   codegen passes [0]
time: 1.308; rss: 104MB LLVM passes
time: 0.000; rss: 104MB serialize work products
time: 0.026; rss: 89MB  linking
    Command being timed: "/home/mark/tmp/master/stage2/bin/rustc -Ztime-passes src/libcore/lib.rs"
    User time (seconds): 35.44
    System time (seconds): 0.22
    Percent of CPU this job got: 99%
    Elapsed (wall clock) time (h:mm:ss or m:ss): 0:35.68
    Average shared text size (kbytes): 0
    Average unshared data size (kbytes): 0
    Average stack size (kbytes): 0
    Average total size (kbytes): 0
    Maximum resident set size (kbytes): 317736
    Average resident set size (kbytes): 0
    Major (requiring I/O) page faults: 1
    Minor (reclaiming a frame) page faults: 115846
    Voluntary context switches: 4
    Involuntary context switches: 58
    Swaps: 0
    File system inputs: 144
    File system outputs: 36016
    Socket messages sent: 0
    Socket messages received: 0
    Signals delivered: 0
    Page size (bytes): 4096
    Exit status: 0
