rust
trait Trait {}
impl Trait for () {}

fn get_trait<I: 'static>() -> impl Trait + 'static {
    ()
}

fn assert_static<A: 'static>(_: A) {}

fn test<T: 'static>(_: T) {
    assert_static(get_trait::<T>());
}
