
error: <inline asm>:1:5: error: unexpected token in operand
        bl *r0
           ^

 --> src/main.rs:9:9
  |
9 |         asm!("bl *$0" : : "r"(foo as fn()));
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
