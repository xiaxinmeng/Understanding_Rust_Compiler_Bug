Rust
trait Xyz {
    type Out: Add<u8>;
}

trait Uvw { /* ... */ }

fn f<T>() -> T {
    unimplemented!()
}

fn foo<T: Uvw>() {
    assert_eq!(f(), 100u8); // failed: cannot infer type for `_`
    assert_eq!(100u8, f()); // passed successfully
}
