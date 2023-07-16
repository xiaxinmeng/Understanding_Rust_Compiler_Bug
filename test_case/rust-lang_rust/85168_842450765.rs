
error[E0532]: expected tuple struct or tuple variant, found type alias `serde_yaml::Result`
 --> src/main.rs:7:9
  |
7 |         serde_yaml::Result(_) => Vec::new(),
  |         ^^^^^^^^^^^^^^^^^^
  |
  = note: can't use a type alias as a constructor
