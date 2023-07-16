
$ cargo build --target wasm32-unknown-unknown
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/[redacted]/rust-lld/build/x86_64-unknown-linux-gnu/stage2/bin/rustc - --crate-name ___ --print=file-names --target wasm32-unknown-unknown --crate-type bin --crate-type rlib` (exit code: 101)
--- stderr
error: Could not create LLVM TargetMachine for triple: wasm32-unknown-unknown-wasm: No available targets are compatible with this triple.
