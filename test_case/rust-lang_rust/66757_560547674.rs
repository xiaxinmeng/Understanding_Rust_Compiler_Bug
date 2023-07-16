rust
#[allow(unreachable_code)]
pub fn foo(never: !) {
    let x: _ = never;
    // Access non-existent field to make the compiler print
    // an error message with the type of `x`
    x.bar;
}
