
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> riscv32imac-unknown-none-elf)
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/home/jonathan/git/rust/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names --target riscv32imac-unknown-none-elf --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro` (exit code: 1)
--- stderr
error: Could not create LLVM TargetMachine for triple: riscv32: No available targets are compatible with triple "riscv32"

command did not execute successfully: "/home/jonathan/git/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "riscv32imac-unknown-none-elf" "-j" "4" "--release" "-p" "alloc" "--manifest-path" "/home/jonathan/git/rust/src/liballoc/Cargo.toml" "--features" "compiler-builtins-mem" "--message-format" "json"
expected success, got: exit code: 101
failed to run: /home/jonathan/git/rust/build/bootstrap/debug/bootstrap build -i --stage 1 src/libcore --target riscv32imac-unknown-none-elf
Build completed unsuccessfully in 0:03:29
