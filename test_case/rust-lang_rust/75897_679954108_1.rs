
error: .seh_ directive must appear within an active frame
 --> lib.rs:5:11
  |
5 |     asm!(".seh_setframe rbp, 16")
  |           ^
  |
note: instantiated into assembly here
 --> <inline asm>:2:2
  |
2 |     .seh_setframe rbp, 16
  |     ^

error: aborting due to previous error
