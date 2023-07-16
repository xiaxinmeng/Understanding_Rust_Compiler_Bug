rust
trait Id<T>: Sized {
    fn id(self) -> T;
}
impl<T> Id<T> for T {
    fn id(self) -> T { self }
}

trait Setup<T> {
    type From: Id<T>;
}

fn transmute<T, U: Setup<T> + ?Sized>(from: U::From) -> T {
    Id::id(from)
}

// compiles fine
pub fn safe_transmute<T, U>(t: T) -> U {
    transmute::<U, dyn Setup<U, From=T>>(t)
}
