
$ rustc out.rs
out.rs:10:20: 10:22 error: borrowed value does not live long enough
out.rs:10     let args = foo(&1);
                             ^~
out.rs:9:11: 11:2 note: reference must be valid for the block at 9:10...
out.rs:9 fn main() {
out.rs:10     let args = foo(&1);
out.rs:11 }
out.rs:10:9: 10:23 note: ...but borrowed value is only valid for the statement at 10:8
out.rs:10     let args = foo(&1);
                  ^~~~~~~~~~~~~~
error: aborting due to previous error
