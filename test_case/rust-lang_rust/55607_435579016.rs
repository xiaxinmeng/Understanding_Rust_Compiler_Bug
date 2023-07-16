rust
>     pub fn new_unchecked_safe(ptr: *mut T) -> Self {
>         NonNull { pointer: NonZero(ptr as _) }
>     }
> 