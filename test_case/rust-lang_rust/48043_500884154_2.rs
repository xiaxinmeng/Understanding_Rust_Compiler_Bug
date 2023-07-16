rust
pub enum ContainerAllocError<E = AllocErr> {
    CapacityOverflow,
    AllocErr {
        layout: Layout,
        error: E,
    },
}

impl SomeContainer<T, A: Alloc> {
    pub fn try_reserve(n: usize) -> Result<(), ContainerAllocError<A::Err>> {…}
}