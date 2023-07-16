
$ nix-shell --packages '(rustChannelOf{ date = "2018-01-27"; channel = "nightly"; }).rust' --run 'rustc --version; cargo clean; export CARGO_INCREMENTAL=0; cargo rustc -- -Ztime-passes'
rustc 1.25.0-nightly (bacb5c58d 2018-01-26)   
   Compiling bare-metal v0.1.1                             
   Compiling vcell v0.1.0                                
   Compiling cortex-m v0.4.3                                          
   Compiling aligned v0.1.1                                         
   Compiling untagged-option v0.1.1                                  
   Compiling volatile-register v0.2.0                              
   Compiling efm32gg11b820 v0.2.0 (file:///home/alex/code/crawford/efm32gg11b820)
  time: 5.444; rss: 1183MB      parsing                            
  time: 0.000; rss: 1183MB      recursion limit                 
  time: 0.000; rss: 1183MB      crate injection                      
  time: 0.000; rss: 1183MB      plugin loading
  time: 0.000; rss: 1183MB      plugin registration
  time: 2.094; rss: 1403MB      expansion
  time: 0.000; rss: 1403MB      maybe building test harness
  time: 0.103; rss: 1403MB      maybe creating a macro crate
  time: 0.329; rss: 1403MB      creating allocators
  time: 0.114; rss: 1403MB      AST validation
  time: 1.686; rss: 1639MB      name resolution
  time: 0.794; rss: 1639MB      complete gated feature checking
  time: 1.511; rss: 2054MB      lowering ast -> hir
  time: 0.437; rss: 2060MB      early lint checks
  time: 1.624; rss: 2145MB      indexing hir
  time: 0.000; rss: 1342MB      load query result cache
  time: 0.000; rss: 1342MB      looking for entry point
  time: 0.005; rss: 1342MB      looking for plugin registrar
  time: 0.097; rss: 1342MB      loop checking
  time: 0.056; rss: 1342MB      static item recursion checking
  time: 0.121; rss: 1422MB      attribute checking
  time: 0.283; rss: 1438MB      stability checking
  time: 2.357; rss: 1858MB      type collecting
  time: 0.034; rss: 1858MB      outlives testing
  time: 0.148; rss: 1859MB      impl wf inference
  time: 8.209; rss: 1869MB      coherence checking
  time: 0.035; rss: 1869MB      variance testing
  time: 4.094; rss: 2237MB      wf checking
  time: 0.725; rss: 2265MB      item-types checking
  time: 20.513; rss: 3064MB     item-bodies checking
  time: 3.108; rss: 3234MB      const checking
  time: 1.842; rss: 3252MB      privacy checking
  time: 0.161; rss: 3252MB      intrinsic checking
  time: 0.656; rss: 3258MB      match checking
  time: 0.280; rss: 3258MB      liveness checking
  time: 15.996; rss: 3936MB     borrow checking
  time: 0.154; rss: 3965MB      MIR borrow checking
  time: 0.075; rss: 3965MB      MIR effect checking
  time: 0.950; rss: 3979MB      death checking
  time: 0.000; rss: 3979MB      unused lib feature checking
  time: 3.065; rss: 3979MB      lint checking
  time: 0.000; rss: 3979MB      resolving dependency formats
    time: 33.326; rss: 4240MB   write metadata
    time: 1.189; rss: 4247MB    translation item collection
    time: 0.158; rss: 4263MB    codegen unit partitioning
    time: 0.039; rss: 4269MB    llvm function passes [efm32gg11b82015]
    time: 0.016; rss: 4269MB    llvm module passes [efm32gg11b82015]
    time: 0.050; rss: 4276MB    llvm function passes [efm32gg11b8200]
    time: 0.016; rss: 4278MB    llvm module passes [efm32gg11b8200]
    time: 0.047; rss: 4286MB    llvm function passes [efm32gg11b8201]
    time: 0.015; rss: 4287MB    llvm module passes [efm32gg11b8201]
    time: 0.989; rss: 4289MB    codegen passes [efm32gg11b82015]
    time: 0.052; rss: 4291MB    llvm function passes [efm32gg11b8202]
    time: 0.012; rss: 4291MB    llvm module passes [efm32gg11b8202]
    time: 0.965; rss: 4286MB    codegen passes [efm32gg11b8200]
    time: 0.043; rss: 4287MB    llvm function passes [efm32gg11b8203]
    time: 0.014; rss: 4287MB    llvm module passes [efm32gg11b8203]
    time: 0.984; rss: 4285MB    codegen passes [efm32gg11b8201]
    time: 0.044; rss: 4287MB    llvm function passes [efm32gg11b8204]
    time: 0.014; rss: 4288MB    llvm module passes [efm32gg11b8204]
    time: 0.756; rss: 4285MB    codegen passes [efm32gg11b8202]
    time: 0.034; rss: 4293MB    llvm function passes [efm32gg11b8205]
    time: 0.011; rss: 4293MB    llvm module passes [efm32gg11b8205]
    time: 0.745; rss: 4292MB    codegen passes [efm32gg11b8203]
    time: 0.068; rss: 4295MB    llvm function passes [efm32gg11b8206]
    time: 0.025; rss: 4296MB    llvm module passes [efm32gg11b8206]
    time: 0.750; rss: 4294MB    codegen passes [efm32gg11b8204]
    time: 0.033; rss: 4295MB    llvm function passes [efm32gg11b8207]
    time: 0.009; rss: 4296MB    llvm module passes [efm32gg11b8207]
    time: 0.614; rss: 4291MB    codegen passes [efm32gg11b8205]
    time: 0.038; rss: 4298MB    llvm function passes [efm32gg11b8208]
    time: 0.009; rss: 4299MB    llvm module passes [efm32gg11b8208]
    time: 0.550; rss: 4296MB    codegen passes [efm32gg11b8206]
    time: 0.466; rss: 4288MB    codegen passes [efm32gg11b8207]
    time: 0.036; rss: 4288MB    llvm function passes [efm32gg11b8209]
    time: 0.013; rss: 4288MB    llvm module passes [efm32gg11b8209]
    time: 0.031; rss: 4295MB    llvm function passes [efm32gg11b82010]
    time: 0.010; rss: 4297MB    llvm module passes [efm32gg11b82010]
    time: 0.489; rss: 4291MB    codegen passes [efm32gg11b8208]
    time: 0.027; rss: 4290MB    llvm function passes [efm32gg11b82011]
    time: 0.007; rss: 4290MB    llvm module passes [efm32gg11b82011]
    time: 0.586; rss: 4294MB    codegen passes [efm32gg11b8209]
    time: 0.035; rss: 4294MB    llvm function passes [efm32gg11b82012]
    time: 0.009; rss: 4295MB    llvm module passes [efm32gg11b82012]
    time: 0.541; rss: 4293MB    codegen passes [efm32gg11b82010]
    time: 0.405; rss: 4285MB    codegen passes [efm32gg11b82011]
    time: 0.031; rss: 4286MB    llvm function passes [efm32gg11b82013]
    time: 0.008; rss: 4286MB    llvm module passes [efm32gg11b82013]
    time: 5.038; rss: 4289MB    translate to LLVM IR
    time: 0.002; rss: 4289MB    assert dep graph
    time: 0.004; rss: 4289MB    serialize dep graph
  time: 41.187; rss: 4289MB     translation
    time: 0.027; rss: 4292MB    llvm function passes [efm32gg11b82014]
    time: 0.013; rss: 4292MB    llvm module passes [efm32gg11b82014]
    time: 0.454; rss: 4287MB    codegen passes [efm32gg11b82012]
    time: 0.446; rss: 3742MB    codegen passes [efm32gg11b82013]
    time: 0.413; rss: 3439MB    codegen passes [efm32gg11b82014]
  time: 5.297; rss: 3423MB      LLVM passes
  time: 0.000; rss: 619MB       serialize work products
  time: 0.120; rss: 551MB       linking
    Finished dev [unoptimized + debuginfo] target(s) in 121.48 secs
