console
$ env CARGO_INCREMENTAL= cargo +nightly rustc -- -Z time-passes
   Compiling rust-issue-43018 v0.1.0 (file:///home/jon/tmp/rust-issue-43018)
time: 0.000; rss: 53MB  parsing
time: 0.000; rss: 53MB  recursion limit
time: 0.000; rss: 53MB  crate injection
time: 0.000; rss: 53MB  plugin loading
time: 0.000; rss: 53MB  plugin registration
time: 0.020; rss: 75MB  expansion
time: 0.000; rss: 75MB  maybe building test harness
time: 0.000; rss: 75MB  maybe creating a macro crate
time: 0.000; rss: 80MB  creating allocators
time: 0.000; rss: 80MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 80MB  early lint checks
time: 0.000; rss: 80MB  AST validation
time: 0.001; rss: 80MB  name resolution
time: 0.000; rss: 80MB  complete gated feature checking
time: 0.000; rss: 80MB  lowering ast -> hir
time: 0.000; rss: 80MB  indexing hir
time: 0.000; rss: 80MB  attribute checking
time: 0.000; rss: 80MB  language item collection
time: 0.000; rss: 80MB  lifetime resolution
time: 0.000; rss: 80MB  looking for entry point
time: 0.000; rss: 80MB  looking for plugin registrar
time: 0.000; rss: 80MB  loop checking
time: 0.000; rss: 80MB  static item recursion checking
time: 0.000; rss: 80MB  compute_incremental_hashes_map
time: 0.000; rss: 80MB  load_dep_graph
time: 0.000; rss: 80MB  stability index
time: 0.000; rss: 80MB  stability checking
time: 0.000; rss: 80MB  type collecting
time: 0.000; rss: 80MB  impl wf inference
time: 0.000; rss: 80MB  coherence checking
time: 0.000; rss: 80MB  variance testing
time: 0.000; rss: 80MB  wf checking
time: 0.000; rss: 80MB  item-types checking
time: 0.031; rss: 106MB item-bodies checking
time: 0.001; rss: 106MB const checking
time: 0.001; rss: 106MB privacy checking
time: 0.000; rss: 106MB intrinsic checking
time: 0.000; rss: 106MB effect checking
time: 0.000; rss: 106MB match checking
time: 0.000; rss: 106MB liveness checking
time: 1.008; rss: 106MB borrow checking
time: 0.000; rss: 106MB reachability checking
time: 0.000; rss: 106MB death checking
time: 0.000; rss: 106MB unused lib feature checking
time: 0.000; rss: 106MB lint checking
time: 0.000; rss: 106MB resolving dependency formats
  time: 0.000; rss: 106MB       write metadata
  time: 16.024; rss: 123MB      translation item collection
  time: 0.001; rss: 123MB       codegen unit partitioning
  time: 0.000; rss: 129MB       write allocator module
time: 46.324; rss: 129MB        translation
time: 0.000; rss: 129MB assert dep graph
time: 0.000; rss: 129MB serialize dep graph
  time: 0.000; rss: 147MB       codegen passes [0]
  time: 0.005; rss: 145MB       codegen passes [1]
  time: 0.014; rss: 147MB       llvm function passes [2]
  time: 0.004; rss: 147MB       llvm module passes [2]
  time: 0.240; rss: 152MB       codegen passes [2]
time: 0.260; rss: 146MB LLVM passes
time: 0.000; rss: 146MB serialize work products
  time: 0.102; rss: 141MB       running linker
time: 0.103; rss: 141MB linking
    Finished dev [unoptimized + debuginfo] target(s) in 47.87 secs
