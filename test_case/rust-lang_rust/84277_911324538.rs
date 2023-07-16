rust
// std
struct Absent<T>(PhantomData<T>);
impl Try for Option<T> {
    type Residual = Absent<T>;
    type Output = T;
    ...
}
impl<A, T> FromResidual<Absent<A>> for Option<T> { ... }
impl<A, T, E> FromResidual<Absent<A>> for Result<T, E> where E: From<Absent<A>> { ... }

// userland
struct Foo;
enum MyError { MissingFoo, MissingOtherType }
impl From<Absent<Foo>> for MyError { ... }
impl From<Absent<OtherType>> for MyError { ... }

fn get_foo() -> Option<Foo> { ... }
fn bar() -> Result<i32, MyError> {
    let foo = get_foo()?;
    Ok(42)
}
