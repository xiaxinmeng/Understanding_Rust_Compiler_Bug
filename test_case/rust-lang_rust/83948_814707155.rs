rust
> // Bad
> let a = 0 as *const u32;
>
> // Good
> let a = std::ptr::null::<u32>();
> 