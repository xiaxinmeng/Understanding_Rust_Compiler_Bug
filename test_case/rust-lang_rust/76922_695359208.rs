
error[E0428]: the name `Foó` is defined multiple times
 --> src/lib.rs:3:1
  |
2 | pub struct Foó; // "Fo\u{f3}"
  | --------------- previous definition of the type `Foó` here
3 | pub struct Foó ; // "Foo\u{301}"
  | ^^^^^^^^^^^^^^^ `Foó` redefined here
  |
  = note: `Foó` must be defined only once in the type namespace of this module
