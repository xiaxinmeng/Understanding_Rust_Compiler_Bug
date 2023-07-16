
error[E0391]: unsupported cyclic reference between types/traits detected
 --> test.rs:5:22
  |
5 |     const X: usize = MyStruct::X + 1;
  |                      ^^^^^^^^^^^ cyclic reference
  |
note: the cycle begins when processing `<impl at test.rs:4:1: 6:2>::X`...
 --> test.rs:5:5
  |
5 |     const X: usize = MyStruct::X + 1;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...which then again requires processing `<impl at test.rs:4:1: 6:2>::X`, completing the cycle.

error[E0391]: unsupported cyclic reference between types/traits detected
 --> test.rs:5:22
  |
5 |     const X: usize = MyStruct::X + 1;
  |                      ^^^^^^^^^^^ cyclic reference
  |
note: the cycle begins when const-evaluating `<impl at test.rs:4:1: 6:2>::X`...
 --> test.rs:5:22
  |
5 |     const X: usize = MyStruct::X + 1;
  |                      ^^^^^^^^^^^
  = note: ...which then again requires const-evaluating `<impl at test.rs:4:1: 6:2>::X`, completing the cycle.

error: aborting due to 2 previous errors
