text
time: 0.000; rss: 53MB  parsing
time: 0.000; rss: 53MB  recursion limit
time: 0.000; rss: 53MB  crate injection
time: 0.000; rss: 53MB  plugin loading
time: 0.000; rss: 53MB  plugin registration
time: 0.027; rss: 80MB  expansion
time: 0.000; rss: 80MB  maybe building test harness
time: 0.000; rss: 84MB  maybe creating a macro crate
time: 0.000; rss: 84MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 84MB  early lint checks
time: 0.000; rss: 84MB  AST validation
time: 0.002; rss: 84MB  name resolution
time: 0.000; rss: 84MB  complete gated feature checking
time: 0.000; rss: 84MB  lowering ast -> hir
time: 0.000; rss: 84MB  indexing hir
time: 0.000; rss: 84MB  attribute checking
time: 0.000; rss: 84MB  language item collection
time: 0.000; rss: 84MB  lifetime resolution
time: 0.000; rss: 84MB  looking for entry point
time: 0.000; rss: 84MB  looking for plugin registrar
time: 0.000; rss: 84MB  loop checking
time: 0.000; rss: 84MB  static item recursion checking
time: 0.000; rss: 84MB  compute_incremental_hashes_map
time: 0.000; rss: 84MB  load_dep_graph
time: 0.000; rss: 84MB  stability index
time: 0.000; rss: 84MB  stability checking
time: 0.000; rss: 84MB  type collecting
time: 0.000; rss: 84MB  impl wf inference
time: 0.000; rss: 84MB  coherence checking
time: 0.000; rss: 84MB  variance testing
time: 0.000; rss: 84MB  wf checking
time: 0.000; rss: 84MB  item-types checking
time: 0.034; rss: 97MB  item-bodies checking
time: 0.001; rss: 97MB  const checking
time: 0.000; rss: 97MB  privacy checking
time: 0.000; rss: 97MB  intrinsic checking
time: 0.000; rss: 97MB  effect checking
time: 0.000; rss: 97MB  match checking
time: 0.000; rss: 97MB  liveness checking
time: 18.628; rss: 104MB        borrow checking
time: 0.000; rss: 104MB reachability checking
time: 0.000; rss: 104MB death checking
time: 0.000; rss: 104MB unused lib feature checking
time: 0.005; rss: 108MB lint checking
time: 0.000; rss: 108MB resolving dependency formats
  time: 0.000; rss: 108MB       write metadata
  time: 472.840; rss: 110MB     translation item collection
  time: 0.001; rss: 112MB       codegen unit partitioning
  time: 0.001; rss: 133MB       internalize symbols
time: 732.741; rss: 133MB       translation
time: 0.000; rss: 133MB assert dep graph
time: 0.000; rss: 133MB serialize dep graph
  time: 0.002; rss: 148MB       codegen passes [0]
  time: 0.014; rss: 146MB       llvm function passes [1]
  time: 0.005; rss: 146MB       llvm module passes [1]
  time: 0.382; rss: 155MB       codegen passes [1]
time: 0.404; rss: 148MB LLVM passes
time: 0.000; rss: 148MB serialize work products
  time: 0.112; rss: 143MB       running linker
time: 0.113; rss: 143MB linking
