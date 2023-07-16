
  $ rustc main.rs -C target-feature=avx
     c/llvm/lib/MC/SubtargetFeature.cpp:197: llvm::FeatureBitse
     llvm::SubtargetFeatures::ApplyFeatureFlag(llvm::FeatureBitset, llvm::StringRef,
     llvm::ArrayRef<llvm::SubtargetFeatureKV>): Assertion `hasFlag(Feature)' failed. 
      Aborted (core dumped)
