
#[cfg(allocator = "jemalloc")]
type GlobalLayout = JemallocLayout;
#[cfg(allocator = "system")]
type GlobalLayout = SystemLayout;
trait AllocatorLayout {
    fn new(...) -> Result<Self>;
    fn set_align(...) -> Result<Self>;
    fn get_align(...) -> usize;
    fn set...
}
