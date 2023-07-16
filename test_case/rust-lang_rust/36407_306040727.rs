
error[E0508]: cannot move out of type `[std::string::String]`, a non-copy array
 --> test.rs:3:9
  |
3 | let y = x[0];
  |         ^^^^
  |         |
  |         help: consider using a reference instead `&x[0]`
  |         cannot move out of here

error: aborting due to previous error(s)
