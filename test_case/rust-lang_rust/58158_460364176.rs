rust
pub struct Matrix<S> {}
pub struct DefaultAllocator;

pub trait Allocator {
    type Buffer;
}

#[repr(packed)]
struct Foo(Matrix<<DefaultAllocator as Allocator>::Buffer>);
