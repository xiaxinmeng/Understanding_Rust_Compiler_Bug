 rust
fn foo(x: &int) -> &int {
    x
}

test.rs:19:5: 19:6 error: cannot infer an appropriate lifetime due to conflicting requirements
test.rs:19     x
               ^
test.rs:19:5: 19:6 error: mismatched types: expected `&int` but found `&int` (lifetime mismatch)
test.rs:19     x
               ^
note: consider using an explicit lifetime parameter as shown:
fn foo<'a>(x: &'a int) -> &'a int
error: aborting due to 2 previous errors
