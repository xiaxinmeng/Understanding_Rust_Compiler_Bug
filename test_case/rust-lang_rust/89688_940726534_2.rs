
error[E0529]: expected an array or slice, found `Vec<_>`
 --> src/main.rs:5:9
  |
5 |     let [single] = Vec::new() else {
  |         ^^^^^^^^ pattern cannot match with input type `Vec<_>`
  |
help: consider slicing here
  |
5 ~     let [single] = Vec::new() else {
6 +         return;
7 +     };[..]
  |
