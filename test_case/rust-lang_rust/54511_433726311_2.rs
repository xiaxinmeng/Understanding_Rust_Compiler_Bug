 console
$ cargo rustc --target msp430-none-elf --release -- --emit=obj
LLVM ERROR: Cannot select: 0x7f6af54571a0: ch = AtomicFence 0x7f6af54703d8, Constant:i16<4>, Constant:i16<0>
  0x7f6af54570d0: i16 = Constant<4>
  0x7f6af5457138: i16 = Constant<0>
In function: _ZN3foo3foo17hdc8504bb86603797E
error: Could not compile `foo`.
