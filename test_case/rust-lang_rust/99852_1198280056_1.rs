rust
fn lambda<T>() -> u32
    where T: Default
{
    let foo: Result<T, ()> = Ok(T::default());
    let baz: u32 = 0;

    if let Ok(foo) = foo && let Ok(bar) = transform(foo) {
        bar
    } else {
        baz
    }
}

fn transform<T, U>(input: T) -> Result<U, ()> {
    todo!()
}
