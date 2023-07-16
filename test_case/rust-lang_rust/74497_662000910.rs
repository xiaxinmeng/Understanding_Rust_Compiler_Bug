
error[E0515]: cannot return value referencing local variable `p`
 --> src/lib.rs:4:26
  |
4 |     foo(|x| { let p = 3; baz(&p) })
  |                          ^^^^--^
  |                          |   |
  |                          |   `p` is borrowed here
  |                          returns a value referencing data owned by the current function
