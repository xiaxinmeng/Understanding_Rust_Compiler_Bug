rust
if let &mut Some(ref mut b) = &mut a.foo {
    a = b;
}
a.foo = None;
