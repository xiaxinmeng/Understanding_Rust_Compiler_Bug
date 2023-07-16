rust
> fn test<T: ?Sized>() {
>     [0u8; std::mem::size_of::<&T>()];
> }
> 