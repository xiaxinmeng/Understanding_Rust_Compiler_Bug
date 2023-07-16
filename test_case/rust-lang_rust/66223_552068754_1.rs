
Compiling rustc-sigsegv v0.1.0 (/Users/ath/Development/rustc-sigsegv)
unknown operand type!
UNREACHABLE executed at /Users/ath/Development/rust/src/llvm-project/llvm/lib/Target/X86/X86AsmPrinter.cpp:207!
error: could not compile `rustc-sigsegv`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name rustc_sigsegv src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C metadata=cfaa57f8238538cc -C extra-filename=-cfaa57f8238538cc --out-dir /Users/ath/Development/rustc-sigsegv/target/release/deps -L dependency=/Users/ath/Development/rustc-sigsegv/target/release/deps -C target-cpu=native` (signal: 6, SIGABRT: process abort signal)
