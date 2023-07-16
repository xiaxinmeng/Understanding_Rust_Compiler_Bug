
------------------------------------------
stderr:
------------------------------------------
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:6:19: 6:20 error: illegal borrow: borrowed value does not live long enough
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:6         let tail = v.view(1, v.len());
                                                                            ^
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:1:54: 9:1 note: borrowed pointer must be valid for the lifetime &r as defined on the block at 1:54...
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:1 fn vec_peek<T>(v: &r/[T]) -> Option< (&r/T, &r/[T]) > {
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:2     if v.len() == 0 {
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:3         None
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:4     } else {
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:5         let head = &v[0];
/Users/tchevalier/rust/src/test/run-pass/issue-3888.rs:6         let tail = v.view(1, v.len());
                                                         ...
note: ...but borrowed value is only valid for unknown scope: 79.  Please report a bug.
error: aborting due to previous error
