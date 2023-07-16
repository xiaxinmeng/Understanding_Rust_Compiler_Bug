rust
trait Is {
    type Type;
    fn into(self) -> Self::Type;
}

impl<T> Is for T {
    type Type = T;
    fn into(self) -> Self::Type {
        self
    }
}

fn foo<T, U>(t: T) -> U
where T: Is<Type = U>
{ t.into() }

fn main() {
    let _: u8 = foo(1u8);
    // Doesn't compile:
    //let _: u16 = foo(1u8);
}
