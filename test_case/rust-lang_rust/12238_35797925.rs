 rust
struct Foo<'x, 'y, 'z> { bar: &'y int, baz: int }
fn foo(x: &Foo) -> (&int, &int, &int) {
    (x.bar, &x.baz, &x.baz)
}

test.rs:24:13: 24:19 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:24     (x.bar, &x.baz, &x.baz)
                       ^~~~~~
test.rs:24:21: 24:27 error: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
test.rs:24     (x.bar, &x.baz, &x.baz)
                               ^~~~~~
test.rs:24:5: 24:28 error: mismatched types: expected `(&int,&int,&int)` but found `(&int,&int,&int)` (lifetime mismatch)
test.rs:24     (x.bar, &x.baz, &x.baz)
               ^~~~~~~~~~~~~~~~~~~~~~~
note: consider using an explicit lifetime parameter as shown:
fn foo<'a, 'b, 'c, 'd>(x: &'d Foo<'b, 'a, 'c>) -> (&'a int, &'d int, &'d int)
error: aborting due to 3 previous errors
