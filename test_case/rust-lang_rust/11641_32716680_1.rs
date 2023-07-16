
test.rs:1:29: 4:2 error: constant contains unimplemented expression type
test.rs:1 static FOO: &'static [u8] = {
test.rs:2     static F: &'static [u8] = &'static [10u8];
test.rs:3     F
test.rs:4 };
error: aborting due to previous error
task 'rustc' failed at 'explicit failure', /home/sfackler/rust/rust/src/libsyntax/diagnostic.rs:75
task '<main>' failed at 'explicit failure', /home/sfackler/rust/rust/src/librustc/lib.rs:448
