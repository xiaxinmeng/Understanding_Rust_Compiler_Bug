
error[E0532]: expected tuple struct or tuple variant, found struct `serde_yaml::Error`
  --> src/main.rs:7:9
   |
7  |         serde_yaml::Error(_) => Vec::new(),
   |         ^^^^^^^^^^^^^^^^^^^^
   | 
  ::: /playground/.cargo/registry/src/github.com-1ecc6299db9ec823/serde_yaml-0.8.17/src/error.rs:14:1
   |
14 | pub struct Error(Box<ErrorImpl>);
   | --------------------------------- `serde_yaml::Error` defined here
   |
help: use struct pattern syntax instead
   |
7  |         serde_yaml::Error { 0 } => Vec::new(),
   |         ^^^^^^^^^^^^^^^^^^^^^^^
help: consider importing one of these items instead
[...]
