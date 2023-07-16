text
error[E0223]: ambiguous associated type
 --> src/main.rs:2:5
  |
2 |     String::from::utf8;
  |     ^^^^^^^^^^^^
  |
help: if there were a trait named `Example` with associated type `from` implemented for `String`, you could use the fully-qualified path
  |
2 |     <String as Example>::from::utf8;
  |     ~~~~~~~~~~~~~~~~~~~~~~~~~
