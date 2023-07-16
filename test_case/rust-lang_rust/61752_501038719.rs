rust
fn breaks2<F>() where F: Foo<Bar: Foo>, F::Bar: Copy {}
