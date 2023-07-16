
nix-shell --packages '(rustChannelOf{ date = "2018-01-28"; channel = "nightly"; }).rust' --run 'rustc --version; cargo clean; export CARGO_INCREMENTAL=0; cargo rustc -
- -Ztime-passes'
rustc 1.25.0-nightly (7d6e5b9da 2018-01-27)
   Compiling cortex-m v0.4.3
   Compiling bare-metal v0.1.1
   Compiling untagged-option v0.1.1
   Compiling vcell v0.1.0
   Compiling aligned v0.1.1
   Compiling volatile-register v0.2.0
   Compiling efm32gg11b820 v0.2.0 (file:///home/alex/code/crawford/efm32gg11b820)
  time: 5.846; rss: 1183MB      parsing
  time: 0.000; rss: 1183MB      recursion limit
  time: 0.000; rss: 1183MB      crate injection
  time: 0.000; rss: 1183MB      plugin loading
  time: 0.000; rss: 1183MB      plugin registration
  time: 2.104; rss: 1404MB      expansion
  time: 0.000; rss: 1404MB      maybe building test harness
  time: 0.097; rss: 1404MB      maybe creating a macro crate
  time: 0.332; rss: 1404MB      creating allocators
  time: 0.121; rss: 1404MB      AST validation
  time: 1.712; rss: 1639MB      name resolution
  time: 0.725; rss: 1639MB      complete gated feature checking
  time: 1.519; rss: 2054MB      lowering ast -> hir
  time: 0.459; rss: 2061MB      early lint checks
  time: 1.674; rss: 2145MB      indexing hir
  time: 0.000; rss: 1342MB      load query result cache
  time: 0.000; rss: 1342MB      looking for entry point
  time: 0.008; rss: 1342MB      looking for plugin registrar
  time: 0.137; rss: 1342MB      loop checking
  time: 0.078; rss: 1342MB      static item recursion checking
  time: 0.151; rss: 1422MB      attribute checking
  time: 0.407; rss: 1439MB      stability checking
  time: 2.691; rss: 1858MB      type collecting
  time: 0.054; rss: 1858MB      outlives testing
  time: 0.190; rss: 1859MB      impl wf inference
  time: 11.854; rss: 1869MB     coherence checking
  time: 0.033; rss: 1869MB      variance testing
  time: 4.392; rss: 2237MB      wf checking
  time: 0.716; rss: 2265MB      item-types checking
  time: 20.862; rss: 3065MB     item-bodies checking
  time: 3.113; rss: 3234MB      const checking
  time: 1.862; rss: 3252MB      privacy checking
  time: 0.159; rss: 3252MB      intrinsic checking
  time: 0.672; rss: 3258MB      match checking
  time: 0.274; rss: 3258MB      liveness checking
  time: 1733.265; rss: 3936MB   borrow checking
  time: 0.150; rss: 3965MB      MIR borrow checking
  time: 0.072; rss: 3965MB      MIR effect checking
  time: 0.943; rss: 3979MB      death checking
  time: 0.000; rss: 3979MB      unused lib feature checking
  time: 3.108; rss: 3979MB      lint checking
  time: 0.000; rss: 3979MB      resolving dependency formats
    time: 36.893; rss: 4239MB   write metadata
    time: 1.178; rss: 4246MB    translation item collection
    time: 0.165; rss: 4262MB    codegen unit partitioning
    time: 0.032; rss: 4268MB    llvm function passes [efm32gg11b82015]
    time: 0.010; rss: 4269MB    llvm module passes [efm32gg11b82015]
    time: 0.049; rss: 4276MB    llvm function passes [efm32gg11b8200]
    time: 0.016; rss: 4277MB    llvm module passes [efm32gg11b8200]
    time: 0.046; rss: 4286MB    llvm function passes [efm32gg11b8201]
    time: 0.016; rss: 4287MB    llvm module passes [efm32gg11b8201]
    time: 0.833; rss: 4291MB    codegen passes [efm32gg11b82015]
    time: 0.043; rss: 4293MB    llvm function passes [efm32gg11b8202]
    time: 0.013; rss: 4293MB    llvm module passes [efm32gg11b8202]
    time: 0.943; rss: 4295MB    codegen passes [efm32gg11b8200]
    time: 0.041; rss: 4298MB    llvm function passes [efm32gg11b8203]
    time: 0.014; rss: 4300MB    llvm module passes [efm32gg11b8203]
    time: 0.850; rss: 4294MB    codegen passes [efm32gg11b8201]
    time: 0.046; rss: 4297MB    llvm function passes [efm32gg11b8204]
    time: 0.014; rss: 4298MB    llvm module passes [efm32gg11b8204]
    time: 0.707; rss: 4288MB    codegen passes [efm32gg11b8202]
    time: 0.034; rss: 4300MB    llvm function passes [efm32gg11b8205]
    time: 0.012; rss: 4301MB    llvm module passes [efm32gg11b8205]
    time: 0.709; rss: 4299MB    codegen passes [efm32gg11b8203]
    time: 0.033; rss: 4301MB    llvm function passes [efm32gg11b8206]
    time: 0.009; rss: 4302MB    llvm module passes [efm32gg11b8206]
    time: 0.669; rss: 4297MB    codegen passes [efm32gg11b8204]
    time: 0.031; rss: 4300MB    llvm function passes [efm32gg11b8207]
    time: 0.009; rss: 4300MB    llvm module passes [efm32gg11b8207]
    time: 0.581; rss: 4299MB    codegen passes [efm32gg11b8205]
    time: 0.039; rss: 4299MB    llvm function passes [efm32gg11b8208]
    time: 0.009; rss: 4300MB    llvm module passes [efm32gg11b8208]
    time: 0.546; rss: 4297MB    codegen passes [efm32gg11b8206]
    time: 0.037; rss: 4306MB    llvm function passes [efm32gg11b8209]
    time: 0.481; rss: 4297MB    codegen passes [efm32gg11b8207]
    time: 0.011; rss: 4295MB    llvm module passes [efm32gg11b8209]
    time: 0.030; rss: 4303MB    llvm function passes [efm32gg11b82010]
    time: 0.010; rss: 4304MB    llvm module passes [efm32gg11b82010]
    time: 0.499; rss: 4296MB    codegen passes [efm32gg11b8208]
    time: 0.026; rss: 4295MB    llvm function passes [efm32gg11b82011]
    time: 0.007; rss: 4296MB    llvm module passes [efm32gg11b82011]
    time: 0.611; rss: 4303MB    codegen passes [efm32gg11b8209]
    time: 0.033; rss: 4302MB    llvm function passes [efm32gg11b82012]
    time: 0.008; rss: 4303MB    llvm module passes [efm32gg11b82012]
    time: 0.549; rss: 4302MB    codegen passes [efm32gg11b82010]
    time: 0.425; rss: 4294MB    codegen passes [efm32gg11b82011]
    time: 0.034; rss: 4294MB    llvm function passes [efm32gg11b82013]
    time: 0.009; rss: 4294MB    llvm module passes [efm32gg11b82013]
    time: 4.866; rss: 4297MB    translate to LLVM IR
    time: 0.002; rss: 4297MB    assert dep graph
    time: 0.000; rss: 4297MB    serialize dep graph
  time: 44.560; rss: 4297MB     translation
    time: 0.043; rss: 4298MB    llvm function passes [efm32gg11b82014]
    time: 0.011; rss: 4299MB    llvm module passes [efm32gg11b82014]
    time: 0.448; rss: 4257MB    codegen passes [efm32gg11b82012]
    time: 0.444; rss: 3870MB    codegen passes [efm32gg11b82013]
    time: 0.406; rss: 3449MB    codegen passes [efm32gg11b82014]
  time: 5.143; rss: 3434MB      LLVM passes
  time: 0.000; rss: 629MB       serialize work products
  time: 0.110; rss: 560MB       linking
    Finished dev [unoptimized + debuginfo] target(s) in 1847.66 secs
