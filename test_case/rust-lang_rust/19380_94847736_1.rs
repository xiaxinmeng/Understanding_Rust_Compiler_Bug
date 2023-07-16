
$ rustc lib.rs
lib.rs:17:33: 17:37 error: cannot convert to a trait object because trait `Qiz` is not object-safe [E0038]
lib.rs:17 const BAR : Bar = Bar { foos: &[&FOO]};
                                          ^~~~
lib.rs:17:33: 17:37 note: method `qiz` has no receiver
lib.rs:17 const BAR : Bar = Bar { foos: &[&FOO]};
                                          ^~~~
error: aborting due to previous error
