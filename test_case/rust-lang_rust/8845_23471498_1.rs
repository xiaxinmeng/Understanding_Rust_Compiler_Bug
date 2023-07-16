
test.rs:23:8: 23:11 error: cannot infer an appropriate lifetime due to conflicting requirements
test.rs:23         Baz { b: self }
                   ^~~
test.rs:22:36: 24:5 note: first, the lifetime cannot outlive the lifetime &'a  as defined on the block at 22:36...
test.rs:22     fn bar<'a>(&'a self) -> Baz<'a> {
test.rs:23         Baz { b: self }
test.rs:24     }
test.rs:23:8: 23:11 note: ...due to the following expression
test.rs:23         Baz { b: self }
                   ^~~
test.rs:22:36: 24:5 note: but, the lifetime must be valid for the lifetime &'self  as defined on the block at 22:36...
test.rs:22     fn bar<'a>(&'a self) -> Baz<'a> {
test.rs:23         Baz { b: self }
test.rs:24     }
test.rs:23:17: 23:23 note: ...due to the following expression
test.rs:23         Baz { b: self }
                            ^~~~~~
error: aborting due to previous error
