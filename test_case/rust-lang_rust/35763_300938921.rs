
error[E0592]: duplicate definitions with name `a`
 --> test.rs:4:3
  |
4 | /   fn a(_x: (), _y: ()) -> () {
5 | |   }
  | |___^ duplicate definitions for `a`
...
8 | /   fn a(_x: ()) -> () {
9 | |   }
  | |___- other definition for `a`

error: aborting due to previous error
