rust
fn inferred_poly<T: Foo>(t: &T) {
    T::foo();
}

fn infer_using_trait_obj(t: &dyn Bar) {
    inferred_poly(t); // Does not work currently
}
