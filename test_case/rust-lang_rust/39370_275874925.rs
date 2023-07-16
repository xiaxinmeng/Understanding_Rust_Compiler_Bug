
$ rustc xx.rs --cfg 'i_dont_exist="foo"'
error[E0463]: can't find crate for `bar`
 --> xx.rs:2:1
  |
2 | extern crate bar;
  | ^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error
