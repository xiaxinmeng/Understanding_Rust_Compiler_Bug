
error[E0311]: the parameter type `R` may not live long enough
 --> test.rs:7:10
  |
7 |         [(|| use_r(vec![0.].into_iter(), r))()]
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: consider adding an explicit lifetime bound for `R`
note: the parameter type `R` must be valid for the anonymous lifetime #1 defined on the body at 6:24...
 --> test.rs:6:25
  |
6 |       let a = |r: &mut R| {
  |  _________________________^
7 | |         [(|| use_r(vec![0.].into_iter(), r))()]
8 | |     };
  | |_____^
note: ...so that the reference type `&mut R` does not outlive the data it points at
 --> test.rs:7:10
  |
7 |         [(|| use_r(vec![0.].into_iter(), r))()]
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
