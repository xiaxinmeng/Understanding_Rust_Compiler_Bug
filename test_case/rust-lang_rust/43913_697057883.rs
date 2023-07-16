
error[E0404]: expected trait, found type alias `Strings`
 --> src/main.rs:3:18
  |
3 | struct Struct<S: Strings>(S);
  |                  ^^^^^^^ type aliases cannot be used as traits
  |
help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
 --> src/main.rs:1:1
  |
1 | type Strings = dyn Iterator<Item=String>;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
