
> cargo rustc --features "everything impl-default" -- -Ztime-passes
   Compiling winapi v0.3.4 (file:///C:/Users/Peter/Code/winapi-rs)
  time: 0.630; rss: 130MB       parsing
  time: 0.000; rss: 130MB       recursion limit
  time: 0.000; rss: 130MB       crate injection
  time: 0.000; rss: 130MB       plugin loading
  time: 0.000; rss: 130MB       plugin registration
  time: 2.792; rss: 472MB       expansion
  time: 0.000; rss: 472MB       maybe building test harness
  time: 0.034; rss: 473MB       maybe creating a macro crate
  time: 0.088; rss: 473MB       creating allocators
  time: 0.107; rss: 473MB       AST validation
  time: 0.364; rss: 512MB       name resolution
  time: 0.116; rss: 512MB       complete gated feature checking
  time: 0.479; rss: 648MB       lowering ast -> hir
  time: 0.131; rss: 648MB       early lint checks
  time: 0.480; rss: 651MB       indexing hir
  time: 0.000; rss: 408MB       load query result cache
  time: 0.000; rss: 408MB       looking for entry point
  time: 0.003; rss: 408MB       looking for plugin registrar
  time: 0.044; rss: 408MB       loop checking
  time: 0.034; rss: 408MB       attribute checking
  time: 0.125; rss: 411MB       stability checking
  time: 0.615; rss: 488MB       type collecting
  time: 0.014; rss: 488MB       outlives testing
  time: 0.032; rss: 492MB       impl wf inference
  time: 0.449; rss: 524MB       coherence checking
  time: 0.016; rss: 524MB       variance testing
  time: 1.237; rss: 663MB       wf checking
  time: 1.406; rss: 730MB       item-types checking
  time: 1.953; rss: 817MB       item-bodies checking
  time: 0.479; rss: 864MB       rvalue promotion
  time: 0.881; rss: 871MB       privacy checking
  time: 0.062; rss: 871MB       intrinsic checking
  time: 0.144; rss: 875MB       match checking
  time: 0.070; rss: 875MB       liveness checking
  time: 2.893; rss: 1040MB      borrow checking
  time: 0.055; rss: 1049MB      MIR borrow checking
  time: 0.029; rss: 1049MB      MIR effect checking
  time: 0.251; rss: 1050MB      death checking
  time: 0.000; rss: 1050MB      unused lib feature checking
  time: 0.804; rss: 1067MB      lint checking
  time: 0.000; rss: 1067MB      dumping chalk-like clauses
  time: 0.000; rss: 1067MB      resolving dependency formats
    time: 3.527; rss: 1149MB    write metadata
    time: 0.014; rss: 1149MB    translation item collection
    time: 0.000; rss: 1149MB    codegen unit partitioning
    time: 0.000; rss: 1150MB    translate to LLVM IR
    time: 0.000; rss: 1150MB    assert dep graph
    time: 0.000; rss: 1150MB    llvm function passes [winapi0]
    time: 0.000; rss: 1151MB    serialize dep graph
    time: 0.000; rss: 1151MB    llvm module passes [winapi0]
  time: 3.548; rss: 1151MB      translation
    time: 0.002; rss: 1138MB    codegen passes [winapi0]
  time: 0.006; rss: 1135MB      LLVM passes
  time: 0.000; rss: 221MB       serialize work products
  time: 0.091; rss: 221MB       linking
    Finished dev [unoptimized + debuginfo] target(s) in 22.8 secs
