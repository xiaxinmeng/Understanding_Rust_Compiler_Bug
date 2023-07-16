
rprichard@ryan:~/mess$ ~/work/rust-staging1/build/install/bin/rustc test.rs
test.rs:2:13: 2:33 error: borrowed value does not live long enough
test.rs:2     let x = format_args!("abc");
                      ^~~~~~~~~~~~~~~~~~~~
note: in expansion of format_args!
test.rs:2:13: 2:33 note: expansion site
test.rs:2:32: 3:2 note: reference must be valid for the block suffix following statement 0 at 2:31...
test.rs:2     let x = format_args!("abc");
test.rs:3 }
test.rs:2:5: 2:32 note: ...but borrowed value is only valid for the statement at 2:4
test.rs:2     let x = format_args!("abc");
              ^~~~~~~~~~~~~~~~~~~~~~~~~~~
test.rs:2:5: 2:32 help: consider using a `let` binding to increase its lifetime
test.rs:2     let x = format_args!("abc");
              ^~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
