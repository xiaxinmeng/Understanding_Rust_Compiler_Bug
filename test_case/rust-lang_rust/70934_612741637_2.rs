
error[E0508]: cannot move out of type `[S; 1]`, a non-copy array
 --> src/lib.rs:4:6
  |
4 |    &([S][0],);
  |      ^^^^^^
  |      |
  |      cannot move out of here
  |      move occurs because value has type `S`, which does not implement the `Copy` trait
