
time: 0.236; rss: 78MB  parsing
time: 0.084; rss: 78MB  configuration 1
time: 0.000; rss: 78MB  recursion limit
time: 0.006; rss: 78MB  gated macro checking
time: 0.000; rss: 78MB  crate injection
time: 0.015; rss: 82MB  macro loading
time: 0.000; rss: 82MB  plugin loading
time: 0.000; rss: 82MB  plugin registration
time: 1.096; rss: 120MB expansion
time: 0.037; rss: 120MB complete gated feature checking 1
time: 0.223; rss: 120MB configuration 2
time: 0.000; rss: 120MB gated configuration checking
time: 0.105; rss: 120MB maybe building test harness
time: 0.099; rss: 121MB prelude injection
time: 0.018; rss: 121MB checking that all macro invocations are gone
time: 0.000; rss: 121MB checking for inline asm in case the target doesn't support it
time: 0.037; rss: 121MB complete gated feature checking 2
time: 0.018; rss: 121MB const fn bodies and arguments
time: 0.099; rss: 121MB assigning node ids
time: 0.164; rss: 189MB lowering ast -> hir
time: 0.052; rss: 199MB indexing hir
time: 0.019; rss: 199MB attribute checking
time: 0.070; rss: 199MB early lint checks
time: 0.014; rss: 232MB external crate/lib resolution
time: 0.016; rss: 236MB language item collection
time: 0.242; rss: 285MB resolution
time: 0.024; rss: 285MB lifetime resolution
time: 0.000; rss: 285MB looking for entry point
time: 0.000; rss: 285MB looking for plugin registrar
time: 0.084; rss: 304MB region resolution
time: 0.016; rss: 304MB loop checking
time: 0.016; rss: 304MB static item recursion checking
time: 0.087; rss: 311MB type collecting
time: 0.003; rss: 311MB variance inference
time: 0.082; rss: 328MB coherence checking
time: 0.490; rss: 333MB wf checking
time: 0.288; rss: 335MB item-types checking
time: 9.299; rss: 465MB item-bodies checking
time: 0.000; rss: 465MB drop-impl checking
time: 0.591; rss: 467MB const checking
time: 0.181; rss: 467MB privacy checking
time: 0.022; rss: 467MB stability index
time: 0.049; rss: 467MB intrinsic checking
time: 0.032; rss: 467MB effect checking
time: 0.157; rss: 467MB match checking
time: 0.087; rss: 467MB liveness checking
time: 0.622; rss: 467MB rvalue checking
time: 0.940; rss: 498MB MIR dump
time: 0.000; rss: 498MB MIR passes
time: 1.090; rss: 500MB borrow checking
time: 0.020; rss: 501MB reachability checking
time: 0.089; rss: 501MB death checking
time: 0.080; rss: 503MB stability checking
time: 0.000; rss: 503MB unused lib feature checking
time: 0.473; rss: 503MB lint checking
time: 0.002; rss: 503MB resolving dependency formats
time: 0.098; rss: 509MB erasing regions from MIR
  time: 2.262; rss: 550MB       translation item collection
  time: 1.118; rss: 1152MB      write metadata
time: 14.097; rss: 1152MB       translation
  time: 5.218; rss: 882MB       llvm function passes [0]
  time: 96.808; rss: 853MB      llvm module passes [0]
          time: 24.214; rss: 857MB      codegen passes [0]
  time: 0.004; rss: 857MB       codegen passes [0]
time: 128.125; rss: 850MB       LLVM passes
  time: 0.343; rss: 850MB       running linker
time: 1.060; rss: 850MB linking
