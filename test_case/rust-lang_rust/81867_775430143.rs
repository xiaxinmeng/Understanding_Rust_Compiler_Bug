rust
trait Trait<T> {
    type Type;
}

impl<T> Trait<T> for u8 {
    type Type = T;
}

struct S<T: Trait<S<T>>>(Option<T::Type>);

fn main() {
    S::<u8>(None); // this line and `cargo build` required for a cycle error
}
