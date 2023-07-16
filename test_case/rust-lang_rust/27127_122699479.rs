
fn foo<T: std::fmt::Display()>(bar: T) {
    unsafe {
        let baz: std::raw::TraitObject = std::mem::transmute(bar);
    }
}
