
warning: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
 --> tests/t1.rs:7:12
  |
7 | #[helper = "::Bar"]
  |            ^^^^^^^ help: use `crate`: `crate::"::Bar"`
  |
