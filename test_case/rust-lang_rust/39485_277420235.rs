rust
fn foo() -> Result<Foo, isize> {
    return Ok(Foo {
        x: Bar { x: 22 },
        a: return Err(32)
    });
}
