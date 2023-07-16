rust
struct DefaultAllocator;

trait Allocator { type Buffer; }

// impl Allocator for DefaultAllocator { type Buffer = (); }

#[repr(packed)]
struct Foo(<DefaultAllocator as Allocator>::Buffer);
