rust
trait Trait {
    type Assoc: 'static;
}

fn get<T: Trait>(val: T::Assoc) -> impl std::any::Any + 'static {
    val
}
