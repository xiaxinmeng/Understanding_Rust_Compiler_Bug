
Samples: 11K of event 'cpu-clock:pppH', Event count (approx.): 2875250000
Overhead  Command  Shared Object                     Symbol
  32.88%  rustc    libLLVM-9-rust-1.43.0-nightly.so  [.] llvm::isSafeToSpeculativelyExecute
  27.12%  rustc    libLLVM-9-rust-1.43.0-nightly.so  [.] llvm::SmallPtrSetImplBase::insert_imp_big
  16.16%  rustc    libLLVM-9-rust-1.43.0-nightly.so  [.] llvm::SmallPtrSetImplBase::Grow
  10.98%  rustc    libLLVM-9-rust-1.43.0-nightly.so  [.] appendSpeculatableOperands
   4.83%  rustc    libLLVM-9-rust-1.43.0-nightly.so  [.] completeEphemeralValues
   1.43%  rustc    libc-2.27.so                      [.] __memset_avx2_erms
   1.31%  rustc    libLLVM-9-rust-1.43.0-nightly.so  [.] canTrapImpl
   1.18%  rustc    libLLVM-9-rust-1.43.0-nightly.so  [.] llvm::CodeMetrics::collectEphemeralValues
   0.57%  rustc    libLLVM-9-rust-1.43.0-nightly.so  [.] (anonymous namespace)::CallAnalyzer::analyzeCall
...
