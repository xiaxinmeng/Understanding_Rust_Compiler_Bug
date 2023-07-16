rust
fn lambda<U>() -> U
    where U: Default
{
    let foo: Result<i32, ()> = Ok(1);
    let baz: U = U::default();

    if let Ok(foo) = foo && let Ok(bar) = transform(foo) {
        bar
    } else {
        baz
    }
}

fn transform<T, U>(input: T) -> Result<U, ()> {
    todo!()
}
