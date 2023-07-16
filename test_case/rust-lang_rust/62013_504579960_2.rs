rust
if let Some(b) = &mut a.foo {
    a = b;
}
a.foo = None;
