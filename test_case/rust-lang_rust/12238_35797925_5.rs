 rust
fn foo<'a, 'b>(x: &'a Foo) -> (&'b int, &'a int, &int) {
    (&x.bar, &x.bar, &x.bar)
}

test.rs:15:6: 15:12 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:15     (&x.bar, &x.bar, &x.bar)
                ^~~~~~
test.rs:15:22: 15:28 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:15     (&x.bar, &x.bar, &x.bar)
                                ^~~~~~
note: consider using an explicit lifetime parameter as shown:
fn foo<'c>(x: &'c Foo) -> (&'c int, &'c int, &'c int)
error: aborting due to 2 previous errors
