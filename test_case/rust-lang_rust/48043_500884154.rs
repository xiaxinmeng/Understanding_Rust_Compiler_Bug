rust
pub enum ContainerAllocError {
    CapacityOverflow,
    AllocErr {
        layout: Layout,
        #[unstable(feature = "container_alloc_error_extra")] error: AllocErr,
    },
}
