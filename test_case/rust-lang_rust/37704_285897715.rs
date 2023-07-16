
$ rustc -C target-feature=_
rustc: /checkout/src/llvm/lib/MC/SubtargetFeature.cpp:194: static void llvm::SubtargetFeatures::ApplyFeatureFlag(llvm::FeatureBitset&, llvm::StringRef, llvm::ArrayRef<llvm::SubtargetFeatureKV>): Assertion `hasFlag(Feature)' failed.
