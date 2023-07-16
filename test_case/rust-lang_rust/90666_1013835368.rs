rust
> struct ArcBuilder<T> { ... }
> impl<T> ArcBuilder<T> {
>     fn new() -> Self;
>     fn get_weak(&self) -> &Weak<T>;
>     fn with_value(self, value: T) -> Arc<T>;
> }
> 