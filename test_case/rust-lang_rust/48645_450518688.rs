text
error[E0515]: cannot return value referencing temporary value
  --> <source>:12:5
   |
12 |     gimme(&(v,))
   |     ^^^^^^^----^
   |     |      |
   |     |      temporary value created here
   |     returns a value referencing data owned by the current function
error: aborting due to previous error
For more information about this error, try `rustc --explain E0515`.
Compiler returned: 1
