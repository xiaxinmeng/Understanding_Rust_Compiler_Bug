
time: 0.548; rss: 71MB  parsing
time: 0.210; rss: 71MB  configuration 1
time: 0.000; rss: 71MB  recursion limit
time: 0.021; rss: 71MB  gated macro checking
time: 0.000; rss: 71MB  crate injection
time: 0.031; rss: 74MB  macro loading
time: 0.000; rss: 74MB  plugin loading
time: 0.000; rss: 74MB  plugin registration
time: 2.268; rss: 109MB expansion
time: 0.096; rss: 109MB complete gated feature checking 1
time: 0.387; rss: 109MB configuration 2
time: 0.000; rss: 109MB gated configuration checking
time: 0.190; rss: 109MB maybe building test harness
time: 0.178; rss: 109MB prelude injection
time: 0.041; rss: 109MB checking that all macro invocations are gone
time: 0.000; rss: 109MB checking for inline asm in case the target doesn't support it
time: 0.077; rss: 109MB complete gated feature checking 2
time: 0.187; rss: 109MB assigning node ids
time: 0.186; rss: 171MB lowering ast -> hir
time: 0.114; rss: 185MB indexing hir
time: 0.000; rss: 185MB attribute checking
time: 0.112; rss: 185MB early lint checks
time: 0.040; rss: 218MB external crate/lib resolution
time: 0.053; rss: 218MB language item collection
time: 0.805; rss: 299MB resolution
time: 0.047; rss: 299MB lifetime resolution
time: 0.000; rss: 299MB looking for entry point
time: 0.026; rss: 299MB looking for plugin registrar
time: 0.212; rss: 321MB region resolution
time: 0.030; rss: 321MB loop checking
time: 0.028; rss: 321MB static item recursion checking
time: 0.269; rss: 329MB type collecting
time: 0.062; rss: 329MB variance inference
time: 0.330; rss: 350MB coherence checking
time: 0.413; rss: 355MB wf checking (old)
time: 0.639; rss: 358MB item-types checking
time: 20.803; rss: 507MB    item-bodies checking
time: 0.000; rss: 507MB drop-impl checking
time: 0.925; rss: 507MB wf checking (new)
time: 1.753; rss: 509MB const checking
time: 0.267; rss: 509MB privacy checking
time: 0.008; rss: 509MB stability index
time: 0.086; rss: 509MB intrinsic checking
time: 0.065; rss: 509MB effect checking
time: 0.271; rss: 509MB match checking
time: 1.709; rss: 638MB MIR dump
time: 0.158; rss: 642MB liveness checking
time: 2.339; rss: 642MB borrow checking
time: 1.892; rss: 643MB rvalue checking
time: 0.092; rss: 643MB reachability checking
time: 0.215; rss: 643MB death checking
time: 0.170; rss: 644MB stability checking
time: 0.000; rss: 644MB unused lib feature checking
time: 1.167; rss: 644MB lint checking
time: 0.004; rss: 644MB resolving dependency formats
time: 40.198; rss: 1388MB   translation
  time: 11.186; rss: 1177MB llvm function passes [0]
  time: 249.835; rss: 1163MB    llvm module passes [0]
  time: 70.738; rss: 1147MB codegen passes [0]
  time: 0.007; rss: 1147MB  codegen passes [0]
time: 337.899; rss: 1147MB  LLVM passes
  time: 0.945; rss: 1149MB  running linker
time: 2.737; rss: 1149MB    linking
