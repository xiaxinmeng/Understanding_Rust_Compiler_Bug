rust
trait Const { const X: i32 }
impl<T> Const for T {
    const X: i32 = match TypeId::of::<T>() {
        // ...
    };
}
