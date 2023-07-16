 rust
struct MyCoolAllocator {
     priv inner: Rc<MyCoolSharedState>
}

impl Allocator for MyCoolAllocator { ... }
