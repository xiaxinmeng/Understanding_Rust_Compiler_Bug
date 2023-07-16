rust
fn foo<T, 'a>(&self, t: &'a mut dyn FnMut() -> T + 'a) -> T {
    t()
}

fn bar<T, 'a>(&self, t: &'a mut dyn FnMut() -> T + 'static) -> T {
    t()
}
