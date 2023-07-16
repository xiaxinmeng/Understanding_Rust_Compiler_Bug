 rust
struct Foo<'x> { bar: int }
fn foo(x: &Foo) -> &int {
    &x.bar
}

test.rs:3:5: 3:11 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:3     &x.bar
              ^~~~~~
note: consider using an explicit lifetime parameter as shown:
fn foo<'a>(x: &'a Foo) -> &'a int
error: aborting due to previous error
