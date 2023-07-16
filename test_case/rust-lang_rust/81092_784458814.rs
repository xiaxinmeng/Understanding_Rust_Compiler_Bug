
error: invalid register `r6`: r6 is used internally by LLVM and cannot be used as an operand for inline asm
   --> arch/cortex-m/src/lib.rs:254:31
    |
254 |     out("r4") _, out("r5") _, out("r6") _, out("r8") _, out("r9") _,
    |                               ^^^^^^^^^^^
