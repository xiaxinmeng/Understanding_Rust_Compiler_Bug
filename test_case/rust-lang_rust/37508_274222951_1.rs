
$ rustc lib.rs --crate-type rlib -C opt-level=3 -C code-model=large
rustc: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/include/llvm/CodeGen/MachineOperand.h:411: int64_t llvm::MachineOperand::getImm() const: Assertion `isImm() && "Wrong MachineOperand accessor"' failed.
