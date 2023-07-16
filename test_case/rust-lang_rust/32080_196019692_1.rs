
time: 0.233; rss: 77MB  parsing
time: 0.084; rss: 77MB  configuration 1
time: 0.000; rss: 77MB  recursion limit
time: 0.006; rss: 77MB  gated macro checking
time: 0.000; rss: 77MB  crate injection
time: 0.014; rss: 82MB  macro loading
time: 0.000; rss: 82MB  plugin loading
time: 0.000; rss: 82MB  plugin registration
time: 1.075; rss: 119MB expansion
time: 0.038; rss: 119MB complete gated feature checking 1
time: 0.226; rss: 119MB configuration 2
time: 0.000; rss: 119MB gated configuration checking
time: 0.106; rss: 119MB maybe building test harness
time: 0.102; rss: 120MB prelude injection
time: 0.019; rss: 120MB checking that all macro invocations are gone
time: 0.000; rss: 120MB checking for inline asm in case the target doesn't support it
time: 0.038; rss: 120MB complete gated feature checking 2
time: 0.018; rss: 120MB const fn bodies and arguments
time: 0.100; rss: 120MB assigning node ids
time: 0.160; rss: 188MB lowering ast -> hir
time: 0.052; rss: 198MB indexing hir
time: 0.019; rss: 198MB attribute checking
time: 0.071; rss: 198MB early lint checks
time: 0.013; rss: 231MB external crate/lib resolution
time: 0.016; rss: 235MB language item collection
time: 0.239; rss: 284MB resolution
time: 0.025; rss: 284MB lifetime resolution
time: 0.000; rss: 284MB looking for entry point
time: 0.000; rss: 284MB looking for plugin registrar
time: 0.082; rss: 302MB region resolution
time: 0.016; rss: 302MB loop checking
time: 0.016; rss: 302MB static item recursion checking
time: 0.086; rss: 310MB type collecting
time: 0.002; rss: 310MB variance inference
time: 0.083; rss: 325MB coherence checking
time: 0.487; rss: 332MB wf checking
time: 0.277; rss: 334MB item-types checking
time: 9.033; rss: 463MB item-bodies checking
time: 0.000; rss: 463MB drop-impl checking
time: 0.581; rss: 464MB const checking
time: 0.181; rss: 464MB privacy checking
time: 0.021; rss: 464MB stability index
time: 0.048; rss: 464MB intrinsic checking
time: 0.032; rss: 464MB effect checking
time: 0.153; rss: 464MB match checking
time: 0.085; rss: 465MB liveness checking
time: 0.613; rss: 465MB rvalue checking
time: 0.930; rss: 497MB MIR dump
time: 0.000; rss: 497MB MIR passes
time: 1.081; rss: 499MB borrow checking
time: 0.020; rss: 500MB reachability checking
time: 0.088; rss: 500MB death checking
time: 0.080; rss: 502MB stability checking
time: 0.000; rss: 502MB unused lib feature checking
time: 0.477; rss: 502MB lint checking
time: 0.002; rss: 502MB resolving dependency formats
time: 0.098; rss: 508MB erasing regions from MIR
  time: 2.258; rss: 549MB       translation item collection
  time: 1.123; rss: 1311MB      write metadata
time: 13.410; rss: 1311MB       translation
  time: 6.678; rss: 1045MB      llvm function passes [0]
  time: 123.685; rss: 1050MB    llvm module passes [0]
  time: 31.072; rss: 1054MB     codegen passes [0]
  time: 0.004; rss: 987MB       codegen passes [0]
time: 163.689; rss: 981MB       LLVM passes
  time: 0.350; rss: 981MB       running linker
time: 1.169; rss: 985MB linking
