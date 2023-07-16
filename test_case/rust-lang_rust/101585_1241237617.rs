
╰ ➤ cargo +master-stage1 run
   Compiling scratch v0.1.0 (/tmp/scratch)
rustc: /checkout/src/llvm-project/llvm/include/llvm/ADT/APInt.h:1469: uint64_t llvm::APInt::getZExtValue() const: Assertion `getActiveBits() <= 64 && "Too many bits for uint64_t"' failed.
error: could not compile `scratch`

Caused by:
  process didn't exit successfully: `rustc --crate-name scratch --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=604a9a6ba3499cae -C extra-filename=-604a9a6ba3499cae --out-dir /tmp/scratch/target/debug/deps -C linker=clang -C incremental=/tmp/scratch/target/debug/incremental -L dependency=/tmp/scratch/target/debug/deps -C link-arg=-fuse-ld=mold` (signal: 6, SIGABRT: process abort signal)
