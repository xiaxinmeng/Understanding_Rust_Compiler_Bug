
warning: the option `Z` is unstable and should only be used on the nightly compiler, but it is currently accepted for backwards compatibility; this will soon change, see issue #31847 for more details

time: 0.000; rss: 15MB  parsing
time: 0.000; rss: 16MB  configuration
time: 0.000; rss: 16MB  recursion limit
time: 0.000; rss: 16MB  crate injection
time: 0.000; rss: 16MB  plugin loading
time: 0.000; rss: 16MB  plugin registration
time: 0.039; rss: 48MB  expansion
time: 0.000; rss: 48MB  maybe building test harness
time: 0.000; rss: 49MB  assigning node ids
time: 0.000; rss: 49MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 49MB  complete gated feature checking
time: 0.000; rss: 49MB  collecting defs
time: 0.003; rss: 49MB  external crate/lib resolution
time: 0.000; rss: 49MB  early lint checks
time: 0.000; rss: 49MB  AST validation
time: 0.000; rss: 49MB  name resolution
time: 0.000; rss: 50MB  lowering ast -> hir
time: 0.000; rss: 50MB  indexing hir
time: 0.000; rss: 50MB  attribute checking
time: 0.000; rss: 50MB  language item collection
time: 0.000; rss: 50MB  lifetime resolution
time: 0.000; rss: 50MB  looking for entry point
time: 0.000; rss: 50MB  looking for plugin registrar
time: 0.000; rss: 50MB  region resolution
time: 0.000; rss: 50MB  loop checking
time: 0.000; rss: 50MB  static item recursion checking
time: 0.000; rss: 50MB  load_dep_graph
time: 0.000; rss: 50MB  type collecting
time: 0.000; rss: 50MB  variance inference
time: 0.006; rss: 51MB  coherence checking
time: 0.000; rss: 52MB  wf checking
time: 0.000; rss: 52MB  item-types checking
time: 0.001; rss: 53MB  item-bodies checking
time: 0.000; rss: 53MB  drop-impl checking
time: 0.000; rss: 53MB  const checking
time: 0.000; rss: 53MB  privacy checking
time: 0.000; rss: 53MB  stability index
time: 0.000; rss: 53MB  intrinsic checking
time: 0.000; rss: 53MB  effect checking
time: 0.000; rss: 53MB  match checking
time: 0.000; rss: 53MB  liveness checking
time: 0.000; rss: 53MB  rvalue checking
time: 0.000; rss: 54MB  MIR dump
time: 0.001; rss: 54MB  MIR passes
time: 0.000; rss: 54MB  borrow checking
time: 0.000; rss: 54MB  reachability checking
time: 0.000; rss: 54MB  death checking
time: 0.000; rss: 54MB  stability checking
time: 0.000; rss: 54MB  unused lib feature checking
time: 0.000; rss: 54MB  lint checking
time: 0.002; rss: 54MB  resolving dependency formats
time: 0.000; rss: 54MB  Prepare MIR codegen passes
  time: 0.000; rss: 55MB  write metadata
  time: 0.001; rss: 55MB  translation item collection
  time: 0.000; rss: 55MB  codegen unit partitioning
  time: 0.000; rss: 57MB  internalize symbols
time: 0.011; rss: 57MB  translation
time: 0.000; rss: 57MB  assert dep graph
time: 0.000; rss: 57MB  serialize dep graph
  time: 0.001; rss: 58MB  llvm function passes [0]
  time: 0.002; rss: 59MB  llvm module passes [0]
  time: 0.003; rss: 62MB  codegen passes [0]
  time: 0.001; rss: 62MB  codegen passes [0]
time: 0.009; rss: 62MB  LLVM passes
time: 0.000; rss: 62MB  serialize work products
  time: 0.161; rss: 62MB  running linker
time: 0.162; rss: 62MB  linking
    Finished release [optimized] target(s) in 1865.86 secs
