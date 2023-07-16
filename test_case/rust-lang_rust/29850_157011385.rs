
time: 0.596; rss: 73MB  parsing
time: 0.176; rss: 73MB  configuration 1
time: 0.000; rss: 73MB  recursion limit
time: 0.017; rss: 73MB  gated macro checking
time: 0.000; rss: 73MB  crate injection
time: 0.028; rss: 75MB  macro loading
time: 0.000; rss: 75MB  plugin loading
time: 0.000; rss: 75MB  plugin registration
time: 2.619; rss: 112MB expansion
time: 0.079; rss: 112MB complete gated feature checking 1
time: 0.477; rss: 113MB configuration 2
time: 0.000; rss: 113MB gated configuration checking
time: 0.232; rss: 113MB maybe building test harness
time: 0.219; rss: 113MB prelude injection
time: 0.042; rss: 113MB checking that all macro invocations are gone
time: 0.000; rss: 113MB checking for inline asm in case the target doesn't support it
time: 0.078; rss: 113MB complete gated feature checking 2
time: 0.207; rss: 113MB assigning node ids
time: 0.208; rss: 178MB lowering ast -> hir
time: 0.139; rss: 192MB indexing hir
time: 0.000; rss: 192MB attribute checking
time: 0.109; rss: 192MB early lint checks
time: 0.042; rss: 224MB external crate/lib resolution
time: 0.059; rss: 224MB language item collection
time: 0.797; rss: 306MB resolution
time: 0.059; rss: 306MB lifetime resolution
time: 0.000; rss: 306MB looking for entry point
time: 0.031; rss: 306MB looking for plugin registrar
time: 0.213; rss: 328MB region resolution
time: 0.028; rss: 328MB loop checking
time: 0.030; rss: 328MB static item recursion checking
time: 0.263; rss: 336MB type collecting
time: 0.064; rss: 336MB variance inference
time: 0.331; rss: 357MB coherence checking
time: 0.364; rss: 362MB wf checking (old)
time: 0.678; rss: 364MB item-types checking
time: 20.812; rss: 514MB    item-bodies checking
time: 0.000; rss: 514MB drop-impl checking
time: 1.031; rss: 514MB wf checking (new)
time: 1.710; rss: 517MB const checking
time: 0.294; rss: 517MB privacy checking
time: 0.010; rss: 517MB stability index
time: 0.095; rss: 517MB intrinsic checking
time: 0.073; rss: 517MB effect checking
time: 0.317; rss: 517MB match checking
time: 1.838; rss: 646MB MIR dump
time: 0.166; rss: 650MB liveness checking
time: 2.271; rss: 650MB borrow checking
time: 1.840; rss: 650MB rvalue checking
time: 0.101; rss: 650MB reachability checking
time: 0.210; rss: 650MB death checking
time: 0.168; rss: 651MB stability checking
time: 0.000; rss: 651MB unused lib feature checking
time: 1.347; rss: 651MB lint checking
time: 0.006; rss: 651MB resolving dependency formats
time: 40.322; rss: 1413MB   translation
  time: 10.886; rss: 1194MB llvm function passes [0]
  time: 257.967; rss: 1180MB    llvm module passes [0]
  time: 69.784; rss: 1164MB codegen passes [0]
  time: 0.006; rss: 1164MB  codegen passes [0]
time: 344.635; rss: 1164MB  LLVM passes
  time: 0.869; rss: 1166MB  running linker
time: 3.216; rss: 1166MB    linking
