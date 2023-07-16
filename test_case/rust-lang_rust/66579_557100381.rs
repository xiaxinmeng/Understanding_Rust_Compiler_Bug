rust
> pub struct StaticVec<T, const N: usize> {
>   data: MaybeUninit<[MaybeUninit<T>; N]>,
>   length: usize,
> }
> 