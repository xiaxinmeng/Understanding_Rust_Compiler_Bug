
export CARGO_INCREMENTAL=0; cargo clean && cargo +nightly-2018-01-27 rustc -- -Ztime-passes
   Compiling cortex-m v0.4.3
   Compiling bare-metal v0.1.1
   Compiling untagged-option v0.1.1
   Compiling aligned v0.1.1
   Compiling vcell v0.1.0
   Compiling volatile-register v0.2.0
   Compiling efm32gg11b820 v0.2.0 (file:///root/efm32gg11b820)
  time: 5.729; rss: 1183MB      parsing
  time: 0.000; rss: 1183MB      recursion limit
  time: 0.000; rss: 1183MB      crate injection
  time: 0.000; rss: 1183MB      plugin loading
  time: 0.000; rss: 1183MB      plugin registration
  time: 2.070; rss: 1404MB      expansion
  time: 0.000; rss: 1404MB      maybe building test harness
  time: 0.110; rss: 1404MB      maybe creating a macro crate
  time: 0.328; rss: 1404MB      creating allocators
  time: 0.109; rss: 1404MB      AST validation
  time: 1.656; rss: 1639MB      name resolution
  time: 0.758; rss: 1639MB      complete gated feature checking
  time: 1.495; rss: 2055MB      lowering ast -> hir
  time: 0.435; rss: 2061MB      early lint checks
  time: 1.623; rss: 2145MB      indexing hir
  time: 0.000; rss: 1344MB      load query result cache
  time: 0.000; rss: 1344MB      looking for entry point
  time: 0.008; rss: 1344MB      looking for plugin registrar
  time: 0.087; rss: 1344MB      loop checking
  time: 0.051; rss: 1343MB      static item recursion checking
  time: 0.125; rss: 1422MB      attribute checking
  time: 0.282; rss: 1438MB      stability checking
  time: 2.361; rss: 1858MB      type collecting
  time: 0.032; rss: 1858MB      outlives testing
  time: 0.143; rss: 1858MB      impl wf inference
  time: 7.993; rss: 1869MB      coherence checking
  time: 0.033; rss: 1869MB      variance testing
  time: 3.987; rss: 2232MB      wf checking
  time: 0.707; rss: 2259MB      item-types checking
  time: 20.370; rss: 3061MB     item-bodies checking
  time: 3.053; rss: 3231MB      const checking
  time: 1.815; rss: 3249MB      privacy checking
  time: 0.156; rss: 3249MB      intrinsic checking
  time: 0.668; rss: 3255MB      match checking
  time: 0.280; rss: 3255MB      liveness checking
  time: 16.005; rss: 3932MB     borrow checking
  time: 0.156; rss: 3961MB      MIR borrow checking
  time: 0.082; rss: 3961MB      MIR effect checking
  time: 0.934; rss: 3975MB      death checking
  time: 0.000; rss: 3975MB      unused lib feature checking
  time: 2.998; rss: 3976MB      lint checking
  time: 0.000; rss: 3976MB      resolving dependency formats
    time: 33.222; rss: 4244MB   write metadata
    time: 1.152; rss: 4251MB    translation item collection
    time: 0.158; rss: 4268MB    codegen unit partitioning
    time: 0.051; rss: 4273MB    llvm function passes [efm32gg11b82015]
    time: 0.026; rss: 4273MB    llvm module passes [efm32gg11b82015]
    time: 0.036; rss: 4280MB    llvm function passes [efm32gg11b8200]
    time: 0.019; rss: 4281MB    llvm module passes [efm32gg11b8200]
    time: 0.048; rss: 4295MB    llvm function passes [efm32gg11b8201]
    time: 0.020; rss: 4296MB    llvm module passes [efm32gg11b8201]
    time: 0.882; rss: 4294MB    codegen passes [efm32gg11b82015]
    time: 0.046; rss: 4299MB    llvm function passes [efm32gg11b8202]
    time: 0.016; rss: 4300MB    llvm module passes [efm32gg11b8202]
    time: 0.893; rss: 4292MB    codegen passes [efm32gg11b8200]
    time: 0.042; rss: 4297MB    llvm function passes [efm32gg11b8203]
    time: 0.025; rss: 4299MB    llvm module passes [efm32gg11b8203]
    time: 0.814; rss: 4291MB    codegen passes [efm32gg11b8201]
    time: 0.040; rss: 4298MB    llvm function passes [efm32gg11b8204]
    time: 0.015; rss: 4299MB    llvm module passes [efm32gg11b8204]
    time: 0.748; rss: 4294MB    codegen passes [efm32gg11b8202]
    time: 0.037; rss: 4308MB    llvm function passes [efm32gg11b8205]
    time: 0.020; rss: 4309MB    llvm module passes [efm32gg11b8205]
    time: 0.745; rss: 4305MB    codegen passes [efm32gg11b8203]
    time: 0.037; rss: 4302MB    llvm function passes [efm32gg11b8206]
    time: 0.010; rss: 4303MB    llvm module passes [efm32gg11b8206]
    time: 0.751; rss: 4300MB    codegen passes [efm32gg11b8204]
    time: 0.038; rss: 4300MB    llvm function passes [efm32gg11b8207]
    time: 0.009; rss: 4301MB    llvm module passes [efm32gg11b8207]
    time: 0.619; rss: 4301MB    codegen passes [efm32gg11b8205]
    time: 0.035; rss: 4303MB    llvm function passes [efm32gg11b8208]
    time: 0.019; rss: 4304MB    llvm module passes [efm32gg11b8208]
    time: 0.611; rss: 4302MB    codegen passes [efm32gg11b8206]
    time: 0.034; rss: 4303MB    llvm function passes [efm32gg11b8209]
    time: 0.013; rss: 4304MB    llvm module passes [efm32gg11b8209]
    time: 0.593; rss: 4304MB    codegen passes [efm32gg11b8207]
    time: 0.032; rss: 4304MB    llvm function passes [efm32gg11b82010]
    time: 0.014; rss: 4304MB    llvm module passes [efm32gg11b82010]
    time: 0.580; rss: 4303MB    codegen passes [efm32gg11b8208]
    time: 0.028; rss: 4301MB    llvm function passes [efm32gg11b82011]
    time: 0.010; rss: 4301MB    llvm module passes [efm32gg11b82011]
    time: 0.591; rss: 4301MB    codegen passes [efm32gg11b8209]
    time: 0.030; rss: 4305MB    llvm function passes [efm32gg11b82012]
    time: 0.011; rss: 4306MB    llvm module passes [efm32gg11b82012]
    time: 0.367; rss: 4307MB    codegen passes [efm32gg11b82011]
    time: 0.578; rss: 4297MB    codegen passes [efm32gg11b82010]
    time: 0.032; rss: 4300MB    llvm function passes [efm32gg11b82013]
    time: 0.012; rss: 4300MB    llvm module passes [efm32gg11b82013]
    time: 5.371; rss: 4305MB    translate to LLVM IR
    time: 0.000; rss: 4305MB    assert dep graph
    time: 0.000; rss: 4305MB    serialize dep graph
  time: 41.314; rss: 4305MB     translation
    time: 0.418; rss: 4297MB    codegen passes [efm32gg11b82012]
    time: 0.034; rss: 4296MB    llvm function passes [efm32gg11b82014]
    time: 0.016; rss: 4296MB    llvm module passes [efm32gg11b82014]
    time: 0.421; rss: 4141MB    codegen passes [efm32gg11b82013]
    time: 0.373; rss: 3692MB    codegen passes [efm32gg11b82014]
  time: 5.537; rss: 3617MB      LLVM passes
  time: 0.000; rss: 636MB       serialize work products
  time: 0.119; rss: 568MB       linking
    Finished dev [unoptimized + debuginfo] target(s) in 121.25 secs
