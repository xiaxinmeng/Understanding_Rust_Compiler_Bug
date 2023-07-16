
error[E0532]: cannot match against a tuple struct which contains private fields
 --> src/main.rs:7:9
  |
7 |         serde_yaml::Error(_) => Vec::new(),
  |         ^^^^^^^^^^^^^^^^^ constructor is not visible here due to private fields
  |
help: consider importing one of these items instead
[...]
