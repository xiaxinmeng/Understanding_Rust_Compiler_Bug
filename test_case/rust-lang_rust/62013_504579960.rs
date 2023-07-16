rust
if let Some(ref mut b) = a.foo {
    a = b;
}
a.foo = None;
