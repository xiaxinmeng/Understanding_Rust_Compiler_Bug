
trait Layout { ... }
trait Allocator {
    type Layout: Layout;
}
struct JemallocLayout { ... }
// JemallocLayout::new -> Result<>
impl Allocator for Jemalloc { type Layout = JemallocLayout; ... }
