rust
trait Trait {
    type Assoc;
}

impl Trait for () {
    type Assoc = ();
}

trait Dummy<T> {}

impl<T, U> Dummy<T> for U {}

fn make<T: Trait>() -> impl Dummy<T::Assoc> {}

fn extract<T>(_: impl Dummy<T>) -> Option<T> {
    None
}

pub fn ice() {
    extract(make::<()>());
}
