
foo.rs:25:21: 25:29 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements [E0495]
foo.rs:25         bar.set_foo(&mut foo);
                              ^~~~~~~~
foo.rs:24:30: 26:6 note: first, the lifetime cannot outlive the lifetime  as defined on the block at 24:29...
foo.rs:24     with_bar(|bar: &mut Bar| {
foo.rs:25         bar.set_foo(&mut foo);
foo.rs:26     });
foo.rs:25:21: 25:29 note: ...so that closure can access `foo`
foo.rs:25         bar.set_foo(&mut foo);
                              ^~~~~~~~
foo.rs:24:30: 26:6 note: but, the lifetime must be valid for the anonymous lifetime #2 defined on the block at 24:29...
foo.rs:24     with_bar(|bar: &mut Bar| {
foo.rs:25         bar.set_foo(&mut foo);
foo.rs:26     });
foo.rs:25:13: 25:20 note: ...so that types are compatible (expected `&mut Bar<'_>`, found `&mut Bar<'_>`)
foo.rs:25         bar.set_foo(&mut foo);
                      ^~~~~~~
error: aborting due to previous error

