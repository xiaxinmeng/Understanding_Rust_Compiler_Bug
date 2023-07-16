rust
pub struct ContainerAllocError {
    layout_of_failed_allocation: Option<Layout>,
}

impl ContainerAllocError {
    pub fn layout_of_failed_allocation(&self) -> Option<Layout> { self.layout_of_failed_allocation }
}
