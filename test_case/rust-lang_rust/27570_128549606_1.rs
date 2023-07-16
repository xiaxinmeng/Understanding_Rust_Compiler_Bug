 rust
/// `Self` has the same size as `T`
trait SizeOf<T> { /* compiler magic */ }

fn transmute<T, U>(x: T) -> U
    where U: SizeOf<T>
