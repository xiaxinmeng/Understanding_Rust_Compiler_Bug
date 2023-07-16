
Before:
   Compiling rustc v0.0.0 (file:///C:/msys64/home/we/rust/src/librustc)
time: 0.214; rss: 60MB  parsing
time: 0.000; rss: 61MB  recursion limit
time: 0.000; rss: 61MB  crate injection
time: 0.000; rss: 61MB  plugin loading
time: 0.000; rss: 61MB  plugin registration
time: 0.894; rss: 179MB expansion
time: 0.000; rss: 179MB maybe building test harness
time: 0.023; rss: 179MB maybe creating a macro crate
time: 0.000; rss: 179MB checking for inline asm in case the target doesn't support it
time: 0.055; rss: 179MB early lint checks
time: 0.024; rss: 179MB AST validation
time: 0.398; rss: 204MB name resolution
time: 0.045; rss: 204MB complete gated feature checking
time: 0.306; rss: 293MB lowering ast -> hir
time: 0.040; rss: 285MB indexing hir
time: 0.017; rss: 285MB attribute checking
time: 0.008; rss: 251MB language item collection
...

After:
   Compiling rustc v0.0.0 (file:///C:/msys64/home/we/rust/src/librustc)
time: 0.220; rss: 64MB  parsing
time: 0.000; rss: 64MB  recursion limit
time: 0.000; rss: 64MB  crate injection
time: 0.000; rss: 64MB  plugin loading
time: 0.000; rss: 64MB  plugin registration
time: 0.844; rss: 186MB expansion
time: 0.000; rss: 186MB maybe building test harness
time: 0.024; rss: 186MB maybe creating a macro crate
time: 0.000; rss: 186MB checking for inline asm in case the target doesn't support it
time: 0.057; rss: 186MB early lint checks
time: 0.027; rss: 186MB AST validation
time: 0.384; rss: 211MB name resolution
time: 0.048; rss: 211MB complete gated feature checking
time: 0.308; rss: 300MB lowering ast -> hir
time: 0.040; rss: 292MB indexing hir
time: 0.018; rss: 292MB attribute checking
time: 0.008; rss: 256MB language item collection
...
