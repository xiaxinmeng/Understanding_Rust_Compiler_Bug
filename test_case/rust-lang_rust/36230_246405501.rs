
# from https://github.com/rust-lang/llvm.git
# I added the tags for clarity
$ git log --graph --decorate=full --oneline llvm-debian/3.8.1-12~..rust-upstream/1.11.0
* 80ad955 (HEAD, tag: refs/tags/rust-upstream/1.11.0) [SimplifyCFG] Don't kill empty cleanuppads with multiple uses
* a73c41e [ArgumentPromotion] Propagate operand bundles to promoted call sites
* 7513452 Fix PR25339: ARM Constant Island
* 80fab33 [WinEH] Don't inline an 'unwinds to caller' cleanupret into funclets which locally unwind
* 25c7dc3 [AA] Hoist the logic to reformulate various AA queries in terms of other parts of the AA interface out of the base class of every single AA result object.
* 361ff3b [PM/AA] Actually wire the AAManager I built for the new pass manager into the new pass manager and fix the latent bugs there.
* 4f2b179 Add an "addUsedAAAnalyses" helper function
* 63f3a1b Add Rust's personality function to the list of known personality functions
* 3dcd2c8 Fix the freebsd building on Windows
* be89e4b Fix cross-compiling to FreeBSD
* deb2692 Don't compile usage of std::thread
* d16f119 [WinEH] Don't perform state stores in cleanups
* 9cc2b72 Add support for i1 compare operations to X86 FastISel, and ignore llvm.assume() intrinsics in the target-independent FastISel.
* 4d5a6d7 Fix compile on older clang/OSX versions
* cca16c0 Add some Rust allocation functions to TargetLibraryInfo + MemoryBuiltins
* de62574 Disable the PassInfo cache assertions to make the cache effective in builds with assertions enabld
* ad57503 (tag: refs/tags/llvm-debian/3.8.1-12) ReleaseNotes: tidy up
