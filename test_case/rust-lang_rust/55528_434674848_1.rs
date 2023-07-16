`
   Compiling unicode-normalization v0.1.7 (/tmp/unicode-normalization)
  time: 0.070; rss: 67MB    parsing
  time: 0.000; rss: 67MB    attributes injection
  time: 0.000; rss: 67MB    recursion limit
  time: 0.000; rss: 67MB    crate injection
  time: 0.000; rss: 67MB    plugin loading
  time: 0.000; rss: 67MB    plugin registration
  time: 0.006; rss: 68MB    pre ast expansion lint checks
    time: 0.032; rss: 86MB  expand crate
    time: 0.000; rss: 86MB  check unused macros
  time: 0.032; rss: 86MB    expansion
  time: 0.000; rss: 86MB    maybe building test harness
  time: 0.001; rss: 86MB    maybe creating a macro crate
  time: 0.004; rss: 87MB    creating allocators
  time: 0.002; rss: 87MB    AST validation
  time: 0.011; rss: 90MB    name resolution
  time: 0.001; rss: 90MB    complete gated feature checking
  time: 0.017; rss: 101MB   lowering ast -> hir
  time: 0.011; rss: 101MB   early lint checks
  time: 0.022; rss: 109MB   indexing hir
  time: 0.000; rss: 96MB    load query result cache
  time: 0.000; rss: 96MB    looking for entry point
  time: 0.000; rss: 96MB    looking for plugin registrar
  time: 0.001; rss: 96MB    loop checking
  time: 0.002; rss: 96MB    attribute checking
  time: 0.004; rss: 96MB    stability checking
  time: 0.012; rss: 114MB   type collecting
  time: 0.000; rss: 114MB   outlives testing
  time: 0.000; rss: 114MB   impl wf inference
  time: 0.026; rss: 122MB   coherence checking
  time: 0.000; rss: 122MB   variance testing
  time: 0.022; rss: 128MB   wf checking
  time: 0.006; rss: 128MB   item-types checking
  time: 11.912; rss: 138MB  item-bodies checking
  time: 0.033; rss: 139MB   rvalue promotion
  time: 0.016; rss: 139MB   privacy checking
  time: 0.002; rss: 139MB   intrinsic checking
  time: 8.093; rss: 146MB   match checking
  time: 0.004; rss: 146MB   liveness checking
  time: 0.261; rss: 170MB   borrow checking
  time: 0.000; rss: 170MB   MIR borrow checking
  time: 0.000; rss: 170MB   dumping chalk-like clauses
  time: 0.000; rss: 170MB   MIR effect checking
  time: 0.004; rss: 170MB   death checking
  time: 0.002; rss: 170MB   unused lib feature checking
  time: 0.012; rss: 170MB   lint checking
  time: 0.000; rss: 170MB   resolving dependency formats
    time: 0.177; rss: 171MB write metadata
      time: 0.000; rss: 171MB   collecting roots
      time: 0.002; rss: 171MB   collecting mono items
    time: 0.002; rss: 171MB monomorphization collection
    time: 0.000; rss: 171MB codegen unit partitioning
    time: 0.007; rss: 174MB codegen to LLVM IR
    time: 0.000; rss: 174MB assert dep graph
    time: 0.000; rss: 174MB serialize dep graph
  time: 0.188; rss: 174MB   codegen
    time: 0.006; rss: 153MB llvm function passes [unicode_normalization.boavajsv-cgu.0]
    time: 0.079; rss: 122MB llvm module passes [unicode_normalization.boavajsv-cgu.0]
    time: 0.114; rss: 128MB codegen passes [unicode_normalization.boavajsv-cgu.0]
  time: 0.207; rss: 127MB   LLVM passes
  time: 0.000; rss: 126MB   serialize work products
  time: 0.005; rss: 124MB   linking
    Finished release [optimized] target(s) in 21.12s
