 rust
use std::mem;
unsafe fn ugh_transmute<T, U>(x: T) -> U {
    assert_eq!(mem::size_of::<T>(), mem::size_of::<U>());
    let y = mem::transmute_copy(&x);
    mem::forget(x);
    y
}
