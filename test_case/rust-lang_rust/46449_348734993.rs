
    time: 0.055; rss: 193MB     write metadata
    time: 3.273; rss: 224MB     translation item collection
    time: 0.017; rss: 232MB     codegen unit partitioning
    time: 4.273; rss: 326MB     translate to LLVM IR
    time: 0.000; rss: 326MB     assert dep graph
    time: 0.000; rss: 326MB     serialize dep graph
  time: 7.792; rss: 326MB       translation
    time: 0.772; rss: 284MB     llvm function passes [shadowsocks0]
    time: 90.584; rss: 6616MB   llvm module passes [shadowsocks0]
Pass Arguments:  -profile-summary-info -module-summary-analysis -write-thinlto-bitcode
Profile summary info
  ModulePass Manager
    Module Summary Analysis
      Unnamed pass: implement Pass::getPassName()
    ThinLTO Bitcode Writer
Pass Arguments:  -domtree -loops -branch-prob -block-freq
  FunctionPass Manager
    Dominator Tree Construction
    Natural Loop Information
    Branch Probability Analysis
    Block Frequency Analysis
