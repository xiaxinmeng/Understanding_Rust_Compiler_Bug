cpp
// Lower an fp-to-int conversion operator from the LLVM opcode, which has an
// undefined result on invalid/overflow, to the WebAssembly opcode, which
// traps on invalid/overflow.
