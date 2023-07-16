
Testing libstd stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-musl)
[01:00:44]    Compiling rand v0.0.0 (file:///checkout/src/librand)
[01:00:44]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:00:44]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[01:00:44]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:00:53]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:01:35] rustc: /checkout/src/llvm/lib/Analysis/ValueTracking.cpp:1594: void computeKnownBits(const llvm::Value*, llvm::APInt&, llvm::APInt&, unsigned int, const {anonymous}::Query&): Assertion `(KnownZero & KnownOne) == 0 && "Bits known to be one AND zero?"' failed.
[01:01:35] error: Could not compile `alloc`.
[01:01:35] warning: build failed, waiting for other jobs to finish...
[01:03:14] error: build failed
