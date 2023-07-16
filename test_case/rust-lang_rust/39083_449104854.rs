text
error[E0669]: invalid value for constraint in inline assembly
  --> src/lib.rs:10:34
   |
10 |     asm!("movw $0, %ss " :: "r" (sel) : "memory");
   |                                  ^^^

error: aborting due to previous error
