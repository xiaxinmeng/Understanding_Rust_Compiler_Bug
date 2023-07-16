sh
invalid expression
!12029 = !DIExpression(6, 34, 0)
invalid expression
!12292 = !DIExpression(6, 34, 0, 6)
invalid expression
!17015 = !DIExpression(6, 34, 8, 6)
invalid expression
!30180 = !DIExpression(6, 34, 8)
rustc: /home/dvc/repos/riscv-rust/llvm/lib/IR/Verifier.cpp:4681: virtual bool {anonymous}::VerifierLegacyPass::doFinalization(llvm::Module&): Assertion `!V->hasBrokenDebugInfo() && "Module contains invalid debug info"' failed.
error: Could not compile `core`.

Caused by:
  process didn't exit successfully: `/home/dvc/repos/riscv-rust/rust/build/bootstrap/debug/rustc --crate-name core src/libcore/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C debug-assertions=off -C overflow-checks=on -C metadata=776197056af321fd -C extra-filename=-776197056af321fd --out-dir /home/dvc/repos/riscv-rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/debug/deps --target x86_64-unknown-linux-gnu -L dependency=/home/dvc/repos/riscv-rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/debug/deps -L dependency=/home/dvc/repos/riscv-rust/rust/build/x86_64-unknown-linux-gnu/stage1-std/debug/deps` (signal: 6, SIGABRT: process abort signal)
thread 'main' panicked at 'command did not execute successfully: "/home/dvc/repos/riscv-rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "--features" "panic-unwind debug-jemalloc jemalloc backtrace" "--manifest-path" "/home/dvc/repos/riscv-rust/rust/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', src/bootstrap/compile.rs:852:8
