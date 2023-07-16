
error: invalid symbol redefinition
  --> src/lib.rs:11:15
   |
11 |         asm!(".Lfoo: nop");
   |               ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:2
   |
2  |     .Lfoo: nop
   |     ^

error: could not compile `playground` due to previous error
