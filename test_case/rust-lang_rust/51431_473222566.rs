none
Compiling playground v0.0.1 (/playground)
error[E0669]: invalid value for constraint in inline assembly
  --> src/main.rs:15:11
   |
15 |     apm!(("al", 0x01), ("bx", 0x00));
   |           ^^^^

error[E0669]: invalid value for constraint in inline assembly
  --> src/main.rs:15:25
   |
15 |     apm!(("al", 0x01), ("bx", 0x00));
   |                         ^^^^

error: <inline asm>:1:11: error: unexpected token in argument list
        mov ah,53h
                 ^

  --> src/main.rs:5:13
   |
5  |             asm! {"mov ah,53h"};
   |             ^^^^^^^^^^^^^^^^^^^^
...
15 |     apm!(("al", 0x01), ("bx", 0x00));
   |     --------------------------------- in this macro invocation

error: <inline asm>:1:8: error: unexpected token in argument list
        int 15h
              ^

  --> src/main.rs:9:13
   |
9  |             asm! {"int 15h"};
   |             ^^^^^^^^^^^^^^^^^
...
15 |     apm!(("al", 0x01), ("bx", 0x00));
   |     --------------------------------- in this macro invocation

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0669`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
