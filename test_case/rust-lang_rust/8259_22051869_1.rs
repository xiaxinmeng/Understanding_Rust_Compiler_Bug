
test.rs:2:32: 2:52 error: cannot infer an appropriate lifetime due to conflicting requirements
test.rs:2 static a: other::Bar<'static> = other::C(&other::A);
                                          ^~~~~~~~~~~~~~~~~~~~
note: first, the lifetime cannot outlive lifetime re_bound(br_self)...
test.rs:2:41: 2:50 note: ...due to the following expression
test.rs:2 static a: other::Bar<'static> = other::C(&other::A);
                                                   ^~~~~~~~~
note: but, the lifetime must be valid for the static lifetime...
test.rs:2:32: 2:52 note: ...due to the following expression
test.rs:2 static a: other::Bar<'static> = other::C(&other::A);
                                          ^~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
