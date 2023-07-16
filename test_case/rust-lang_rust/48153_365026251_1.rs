
export CARGO_INCREMENTAL=0; cargo clean && cargo +nightly-2018-01-28 rustc -- -Ztime-passes
   Compiling untagged-option v0.1.1
   Compiling vcell v0.1.0
   Compiling aligned v0.1.1
   Compiling bare-metal v0.1.1
   Compiling cortex-m v0.4.3
   Compiling volatile-register v0.2.0
   Compiling efm32gg11b820 v0.2.0 (file:///root/efm32gg11b820)
  time: 5.959; rss: 1183MB      parsing
  time: 0.000; rss: 1183MB      recursion limit
  time: 0.000; rss: 1183MB      crate injection
  time: 0.000; rss: 1183MB      plugin loading
  time: 0.000; rss: 1183MB      plugin registration
  time: 2.029; rss: 1404MB      expansion
  time: 0.000; rss: 1404MB      maybe building test harness
  time: 0.098; rss: 1404MB      maybe creating a macro crate
  time: 0.314; rss: 1404MB      creating allocators
  time: 0.104; rss: 1404MB      AST validation
  time: 1.622; rss: 1639MB      name resolution
  time: 0.704; rss: 1639MB      complete gated feature checking
  time: 1.456; rss: 2055MB      lowering ast -> hir
  time: 0.421; rss: 2061MB      early lint checks
  time: 1.578; rss: 2146MB      indexing hir
  time: 0.000; rss: 1344MB      load query result cache
  time: 0.000; rss: 1344MB      looking for entry point
  time: 0.007; rss: 1344MB      looking for plugin registrar
  time: 0.085; rss: 1344MB      loop checking
  time: 0.048; rss: 1343MB      static item recursion checking
  time: 0.119; rss: 1422MB      attribute checking
  time: 0.275; rss: 1439MB      stability checking
  time: 2.313; rss: 1858MB      type collecting
  time: 0.030; rss: 1858MB      outlives testing
  time: 0.137; rss: 1858MB      impl wf inference
  time: 8.591; rss: 1869MB      coherence checking
  time: 0.031; rss: 1869MB      variance testing
  time: 4.028; rss: 2232MB      wf checking
  time: 0.715; rss: 2259MB      item-types checking
  time: 20.167; rss: 3061MB     item-bodies checking
  time: 3.028; rss: 3230MB      const checking
  time: 1.775; rss: 3248MB      privacy checking
  time: 0.152; rss: 3248MB      intrinsic checking
  time: 0.641; rss: 3255MB      match checking
  time: 0.264; rss: 3255MB      liveness checking
  time: 1795.120; rss: 3921MB   borrow checking
  time: 0.169; rss: 3950MB      MIR borrow checking
  time: 0.084; rss: 3948MB      MIR effect checking
  time: 1.005; rss: 3962MB      death checking
  time: 0.000; rss: 3962MB      unused lib feature checking
  time: 3.779; rss: 3962MB      lint checking
  time: 0.000; rss: 3962MB      resolving dependency formats
    time: 41.354; rss: 4213MB   write metadata
    time: 1.431; rss: 4200MB    translation item collection
    time: 0.169; rss: 4219MB    codegen unit partitioning
    time: 0.033; rss: 4235MB    llvm function passes [efm32gg11b82015]
    time: 0.019; rss: 4235MB    llvm module passes [efm32gg11b82015]
    time: 0.035; rss: 4245MB    llvm function passes [efm32gg11b8200]
    time: 0.019; rss: 4246MB    llvm module passes [efm32gg11b8200]
    time: 0.052; rss: 4264MB    llvm function passes [efm32gg11b8201]
    time: 0.016; rss: 4266MB    llvm module passes [efm32gg11b8201]
    time: 0.908; rss: 4254MB    codegen passes [efm32gg11b82015]
    time: 0.044; rss: 4257MB    llvm function passes [efm32gg11b8202]
    time: 0.013; rss: 4258MB    llvm module passes [efm32gg11b8202]
    time: 0.918; rss: 4252MB    codegen passes [efm32gg11b8200]
    time: 0.079; rss: 4257MB    llvm function passes [efm32gg11b8203]
    time: 0.055; rss: 4258MB    llvm module passes [efm32gg11b8203]
    time: 1.252; rss: 4257MB    codegen passes [efm32gg11b8201]
    time: 0.040; rss: 4259MB    llvm function passes [efm32gg11b8204]
    time: 0.013; rss: 4260MB    llvm module passes [efm32gg11b8204]
    time: 1.128; rss: 4255MB    codegen passes [efm32gg11b8202]
    time: 0.042; rss: 4265MB    llvm function passes [efm32gg11b8205]
    time: 0.013; rss: 4265MB    llvm module passes [efm32gg11b8205]
    time: 0.895; rss: 4266MB    codegen passes [efm32gg11b8203]
    time: 0.034; rss: 4264MB    llvm function passes [efm32gg11b8206]
    time: 0.011; rss: 4265MB    llvm module passes [efm32gg11b8206]
    time: 0.728; rss: 4262MB    codegen passes [efm32gg11b8204]
    time: 0.033; rss: 4263MB    llvm function passes [efm32gg11b8207]
    time: 0.011; rss: 4263MB    llvm module passes [efm32gg11b8207]
    time: 0.729; rss: 4265MB    codegen passes [efm32gg11b8205]
    time: 0.057; rss: 4269MB    llvm function passes [efm32gg11b8208]
    time: 0.029; rss: 4271MB    llvm module passes [efm32gg11b8208]
    time: 0.747; rss: 4262MB    codegen passes [efm32gg11b8206]
    time: 0.708; rss: 4261MB    codegen passes [efm32gg11b8207]
    time: 0.034; rss: 4257MB    llvm function passes [efm32gg11b8209]
    time: 0.009; rss: 4257MB    llvm module passes [efm32gg11b8209]
    time: 0.644; rss: 4258MB    codegen passes [efm32gg11b8208]
    time: 0.036; rss: 4256MB    llvm function passes [efm32gg11b82010]
    time: 0.010; rss: 4257MB    llvm module passes [efm32gg11b82010]
    time: 0.063; rss: 4261MB    llvm function passes [efm32gg11b82011]
    time: 0.021; rss: 4261MB    llvm module passes [efm32gg11b82011]
    time: 1.096; rss: 4261MB    codegen passes [efm32gg11b8209]
    time: 0.055; rss: 4262MB    llvm function passes [efm32gg11b82012]
    time: 0.017; rss: 4262MB    llvm module passes [efm32gg11b82012]
    time: 1.037; rss: 4264MB    codegen passes [efm32gg11b82010]
    time: 0.754; rss: 4257MB    codegen passes [efm32gg11b82011]
    time: 0.047; rss: 4258MB    llvm function passes [efm32gg11b82013]
    time: 0.011; rss: 4259MB    llvm module passes [efm32gg11b82013]
    time: 6.638; rss: 4262MB    translate to LLVM IR
    time: 0.572; rss: 4257MB    codegen passes [efm32gg11b82012]
    time: 0.005; rss: 4256MB    assert dep graph
    time: 0.000; rss: 4256MB    serialize dep graph
  time: 51.306; rss: 4255MB     translation
    time: 0.031; rss: 4255MB    llvm function passes [efm32gg11b82014]
    time: 0.010; rss: 4256MB    llvm module passes [efm32gg11b82014]
    time: 0.468; rss: 3666MB    codegen passes [efm32gg11b82013]
    time: 0.412; rss: 3411MB    codegen passes [efm32gg11b82014]
  time: 6.860; rss: 3395MB      LLVM passes
  time: 0.000; rss: 594MB       serialize work products
  time: 0.125; rss: 526MB       linking
    Finished dev [unoptimized + debuginfo] target(s) in 1911.58 secs
