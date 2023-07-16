 rust
fn foo(x: &Foo) -> (&int, &int) {
    (&x.bar, &x.bar)
}

test.rs:11:6: 11:12 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:11     (&x.bar, &x.bar)
                ^~~~~~
test.rs:11:14: 11:20 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:11     (&x.bar, &x.bar)
                        ^~~~~~
note: consider using an explicit lifetime parameter as shown:
fn foo<'a>(x: &'a Foo) -> (&'a int, &'a int)
error: aborting due to 2 previous errors
