
fn fails<T: Trait>() -> T::A where T::A: From<Vec<T::A>> {}
