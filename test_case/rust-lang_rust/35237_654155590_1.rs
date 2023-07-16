
error[E0576]: cannot find associated type `Bar` in trait `Bar`
 --> src/lib.rs:4:39
  |
4 | trait Bar: Foo + AsRef<<Self as Bar>::Bar> {}
  |                                       ^^^ not found in `Bar`
