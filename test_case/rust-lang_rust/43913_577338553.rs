
error[E0404]: expected trait, found type alias `Strings`
 --> file.rs:3:18
  |
3 | struct Struct<S: Strings>(S);
  |                  ^^^^^^^ type aliases cannot be used as traits
  |
  = note: did you mean to use a trait alias?
