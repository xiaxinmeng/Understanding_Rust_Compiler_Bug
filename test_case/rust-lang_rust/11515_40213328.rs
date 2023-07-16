
foo.rs:6:30: 6:39 error: borrowed value does not live long enough
foo.rs:6     let test = ~Test { func: proc() {} };
                                      ^~~~~~~~~
foo.rs:5:11: 7:2 note: reference must be valid for the block at 5:10...
foo.rs:5 fn main() {
foo.rs:6     let test = ~Test { func: proc() {} };
foo.rs:7 }
foo.rs:6:9: 6:41 note: ...but borrowed value is only valid for the statement at 6:8
foo.rs:6     let test = ~Test { func: proc() {} };
                 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
