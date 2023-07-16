
rustc: /checkout/src/llvm-project/llvm/include/llvm/Support/Casting.h:566: decltype(auto) llvm::cast(const From &) [To = llvm::PHINode, From = llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, true, false, void>, false, false>]: Assertion `isa<To>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
warning: `scratch` (bin "scratch" test) generated 3 warnings
error: could not compile `scratch`; 3 warnings emitted

Caused by:
  process didn't exit successfully: `rustc --crate-name scratch --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C opt-level=3 -C embed-bitcode=no --test -C metadata=b0e11864599800ac -C extra-filename=-b0e11864599800ac --out-dir /tmp/scratch/target/release/deps -C linker=clang -L dependency=/tmp/scratch/target/release/deps -C link-arg=-fuse-ld=lld` (signal: 6, SIGABRT: process abort signal)
