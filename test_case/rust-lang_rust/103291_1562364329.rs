rust
trait A {
    const ID: TypeId;
}
fn foo<T: A>(id: TypeId) {
    match id {
        T::ID => {}
        _ => {}
    }
}
