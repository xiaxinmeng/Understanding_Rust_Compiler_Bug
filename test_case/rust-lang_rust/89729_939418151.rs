rust
core::mem::MaybeUninit<T>   const fn uninit() -> MaybeUninit<T>;
core::mem::MaybeUninit<T>   const fn uninit_array<const LEN: usize>() -> [Self; LEN];
core::mem::MaybeUninit<T>   fn zeroed() -> MaybeUninit<T>;
