
error[E0229]: associated type bindings are not allowed here
 --> issue-85350.rs:3:6
  |
3 | impl FnMut(&Context) for 'tcx {
  |      ^^^^^^^^^^^^^^^ associated type not allowed here
