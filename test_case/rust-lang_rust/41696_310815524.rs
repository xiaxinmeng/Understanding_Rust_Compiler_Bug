
time: 0.007; rss: 55MB  parsing
time: 0.000; rss: 55MB  recursion limit
time: 0.000; rss: 55MB  crate injection
time: 0.000; rss: 55MB  plugin loading
time: 0.000; rss: 55MB  plugin registration
time: 0.099; rss: 112MB expansion
time: 0.003; rss: 112MB maybe building test harness
time: 0.000; rss: 112MB maybe creating a macro crate
time: 0.000; rss: 112MB checking for inline asm in case the target doesn't support it
time: 0.001; rss: 112MB early lint checks
time: 0.000; rss: 112MB AST validation
time: 0.011; rss: 120MB name resolution
time: 0.002; rss: 120MB complete gated feature checking
time: 0.004; rss: 122MB lowering ast -> hir
time: 0.001; rss: 122MB indexing hir
time: 0.000; rss: 122MB attribute checking
time: 0.000; rss: 122MB language item collection
time: 0.000; rss: 122MB lifetime resolution
time: 0.000; rss: 122MB looking for entry point
time: 0.000; rss: 122MB looking for plugin registrar
time: 0.000; rss: 122MB loop checking
time: 0.000; rss: 122MB static item recursion checking
time: 0.005; rss: 122MB compute_incremental_hashes_map
time: 0.000; rss: 122MB load_dep_graph
time: 0.000; rss: 122MB stability index
time: 0.001; rss: 122MB stability checking
time: 0.009; rss: 132MB type collecting
time: 0.000; rss: 132MB impl wf inference
time: 0.031; rss: 154MB coherence checking
time: 0.000; rss: 154MB variance testing
time: 0.006; rss: 154MB wf checking
time: 0.006; rss: 158MB item-types checking
time: 0.205; rss: 164MB item-bodies checking
time: 0.021; rss: 168MB const checking
time: 0.001; rss: 168MB privacy checking
time: 0.000; rss: 168MB intrinsic checking
time: 0.000; rss: 168MB effect checking
time: 0.003; rss: 168MB match checking
time: 0.001; rss: 168MB liveness checking
time: 0.070; rss: 174MB borrow checking
time: 0.000; rss: 174MB reachability checking
time: 0.001; rss: 174MB death checking
time: 0.000; rss: 174MB unused lib feature checking
time: 0.014; rss: 174MB lint checking
time: 0.000; rss: 174MB resolving dependency formats
  time: 0.001; rss: 174MB       write metadata
  time: 495.029; rss: 206MB     translation item collection
  time: 0.012; rss: 214MB       codegen unit partitioning
  time: 0.007; rss: 337MB       internalize symbols
time: 807.783; rss: 337MB       translation
time: 0.000; rss: 337MB assert dep graph
time: 0.000; rss: 337MB serialize dep graph
  time: 0.002; rss: 324MB       codegen passes [0]
  time: 0.205; rss: 326MB       llvm function passes [1]
  time: 0.129; rss: 330MB       llvm module passes [1]
  time: 4.515; rss: 336MB       codegen passes [1]
time: 4.852; rss: 332MB LLVM passes
time: 0.000; rss: 332MB serialize work products
  time: 0.682; rss: 245MB       running linker
time: 0.685; rss: 245MB linking
