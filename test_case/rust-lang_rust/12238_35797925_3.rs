 rust
fn foo<'a, 'b>(x: &'a Foo) -> &'b int {
    &x.bar
}

test.rs:7:5: 7:11 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:7     &x.bar
              ^~~~~~
note: consider using an explicit lifetime parameter as shown:
fn foo<'c>(x: &'c Foo) -> &'c int
error: aborting due to previous error
