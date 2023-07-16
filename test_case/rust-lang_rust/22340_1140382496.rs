rust
fn main() {
    fn cast<T>(x: T) -> T
        where T: for<'a> Fn(&'a SimpleWrapper) -> &'a SimpleWrapper
    {
        x
    }

    let x = SimpleWrapper { value: 10 };
    let foo = cast(|wrapper: &SimpleWrapper| wrapper);
    let y = foo(&x);
}

struct SimpleWrapper {
    value: i32,
}
