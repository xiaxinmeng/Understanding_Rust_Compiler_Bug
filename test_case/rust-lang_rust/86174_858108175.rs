rust
> #[repr(transparent)]
> struct W<T>(T);
> 
> const INVALID_VTABLE_SIZE: W<&dyn Send> =
>     unsafe { std::mem::transmute((&92u8, &[1usize, usize::MAX, 0usize])) };
> 