
note: the struct import `A` is defined here...
 --> src/main.rs:2:9
  |
2 |     use bar::A;
  |         ^^^^^^
note: ...and refers to the struct `A` which is defined here
 --> src/main.rs:5:9
  |
5 |         pub struct A {}
  |         ^^^^^^^^^^^^ consider importing it directly
