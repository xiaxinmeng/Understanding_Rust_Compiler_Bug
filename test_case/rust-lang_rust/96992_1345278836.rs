rust
> impl Waker {
>     unsafe fn with_raw<T>(raw: &RawWaker, f: impl FnOnce(&Waker) -> T) -> T;
> }
> 