 rust
$ rustc -O --emit=obj --target riscv32imac-unknown-none riscv.rs
LLVM ERROR: Cannot select: 0x7f2e4c036840: i32,ch = AtomicSwap<(volatile load store seq_cst 4 on %ir.0)> 0x7f2e4c01f418, 0x7f2e4c036708, 0x7f2e4c0367d8
  0x7f2e4c036708: i32,ch = CopyFromReg 0x7f2e4c01f418, Register:i32 %0
    0x7f2e4c0366a0: i32 = Register %0
  0x7f2e4c0367d8: i32,ch = CopyFromReg 0x7f2e4c01f418, Register:i32 %1
    0x7f2e4c036770: i32 = Register %1
In function: foo
