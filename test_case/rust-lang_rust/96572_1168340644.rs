rust
#[allow(unconditional_recursion)]
fn foo(b: bool) -> impl Copy {
    let (mut x, mut y) = foo(false);
    x = 42;
    y = "foo";
    if b {
        panic!()
    } else {
        foo(true)
    }
}
