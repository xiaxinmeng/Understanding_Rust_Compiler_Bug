
error[E0596]: cannot borrow immutable local variable `foo` as mutable
  --> test.rs:11:10
   |
5  | |             *&mut $s = 0;
   | |____________________^ cannot borrow mutably
...
11 |       bad!(foo whatever);
   |  _____-----^-------------
   | |     |
   | |     in this macro invocation

error: aborting due to previous error(s)
