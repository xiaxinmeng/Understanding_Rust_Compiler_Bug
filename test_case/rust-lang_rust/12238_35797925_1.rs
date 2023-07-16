 rust
fn foo<'a, 'b, 'c>(x: &Foo<'a, 'b, 'c>) -> (&int, &int, &int) {
    (x.bar, &x.baz, &x.baz)
}

test.rs:28:13: 28:19 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:28     (x.bar, &x.baz, &x.baz)
                       ^~~~~~
test.rs:28:21: 28:27 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:28     (x.bar, &x.baz, &x.baz)
                               ^~~~~~
test.rs:28:5: 28:28 error: mismatched types: expected `(&int,&int,&int)` but found `(&'b int,&int,&int)` (lifetime mismatch)
test.rs:28     (x.bar, &x.baz, &x.baz)
               ^~~~~~~~~~~~~~~~~~~~~~~
note: consider using an explicit lifetime parameter as shown:
fn foo<'d, 'e, 'a, 'c>(x: &'e Foo<'a, 'd, 'c>) -> (&'d int, &'e int, &'e int)
