rust
fn breaks3_no_atb<F>() where F: Foo, Foo::Bar: Foo, F::Bar: Copy {}
