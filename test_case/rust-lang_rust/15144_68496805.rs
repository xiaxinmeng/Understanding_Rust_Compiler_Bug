 rust
test.rs:25:21: 25:29 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:25         bar.set_foo(&mut foo);
                               ^~~~~~~~
test.rs:21:1: 27:2 help: consider using an explicit lifetime parameter as shown: fn main<'a>()
test.rs:21 fn main() {
test.rs:22     let mut foo = Foo { n : 4 };
test.rs:23
test.rs:24     with_bar(|bar: &mut Bar| {
test.rs:25         bar.set_foo(&mut foo);
test.rs:26     });
           ...
error: aborting due to previous error
