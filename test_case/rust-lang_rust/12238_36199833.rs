 rust
test.rs:2:1: 4:2 note: consider using an explicit lifetime parameter as shown: fn foo1<'a>(x: &'a Foo) -> &'a int
test.rs:2 fn foo1(x: &Foo) -> &int {
test.rs:3     &x.bar
test.rs:4 }
test.rs:3:5: 3:11 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:3     &x.bar
              ^~~~~~
