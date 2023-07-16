text
error[E0432]: unresolved import `self::rand`
 --> src\main.rs:3:15
  |
3 |     use self::rand::Rng;
  |               ^^^^ maybe a missing `extern crate rand;`?

warning: unused import: `self::rand::Rng`
 --> src\main.rs:3:9
  |
3 |     use self::rand::Rng;
  |         ^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default
