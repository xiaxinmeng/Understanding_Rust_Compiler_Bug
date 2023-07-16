rust
const fn fit_array_into<T>() -> usize {
    core::mem::size_of::<T>()
}

struct SmallArray<T: Sized>([core::mem::MaybeUninit<T>; fit_array_into::<T>()]);
