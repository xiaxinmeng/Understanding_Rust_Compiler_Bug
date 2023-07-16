 rust
trait Id_ {
    type Out;
}

type Id<T> = <T as Id_>::Out;

impl<T> Id_ for T {
    default type Out = T;
}

fn main() {
    let x: Id<bool> = panic!();
}
