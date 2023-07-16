rust
pub trait OwningAlloc: Alloc {
    fn owns(&self, ptr: *mut u8, layout: Layout) -> bool;
}
