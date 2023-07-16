
error[E0428]: the name `Abc` is defined multiple times
 --> src/main.rs:3:1
  |
1 | struct Abc(u8);
  | --------------- previous definition of the value `Abc` here
2 | 
3 | fn Abc(_: u8) {}
  | ^^^^^^^^^^^^^ `Abc` redefined here
  |
  = note: `Abc` must be defined only once in the value namespace of this module
