
LLVM ERROR: Not supported instr: <MCInst 258 <MCOperand Reg:1> <MCOperand Imm:15> <MCOperand Reg:45>>
error: could not compile `compiler_builtins`

Caused by:
  process didn't exit successfully: `rustc --crate-name compiler_builtins /Users/***/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.39/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C panic=abort -C embed-bitcode=no --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=358757d918a0ced2 -C extra-filename=-358757d918a0ced2 --out-dir /Users/***/ctrlr/target/avr-unknown-gnu-atmega2560/release/deps --target /Users/***/ctrlr/avr-unknown-gnu-atmega2560.json -Z force-unstable-if-unmarked -L dependency=/Users/***/ctrlr/target/avr-unknown-gnu-atmega2560/release/deps -L dependency=/Users/***/ctrlr/target/release/deps --extern core=/Users/***/ctrlr/target/avr-unknown-gnu-atmega2560/release/deps/librustc_std_workspace_core-0fe1efb82ffd1726.rmeta --cap-lints allow --cfg 'feature="unstable"'` (exit code: 101)
