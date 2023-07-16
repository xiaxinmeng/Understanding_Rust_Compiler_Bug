rust
pub enum ContainerAllocError {
    CapacityOverflow,
    AllocErr { layout: Layout },
}
