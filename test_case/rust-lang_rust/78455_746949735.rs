rust
> fn try_map<F, U, R>(orig: Ref<'b, T>, f: F) -> Result<Ref<'b, U>, R::Error>
> where
>     F: FnOnce(&T) -> R,
>     R: Try<Ok = &U>
> 