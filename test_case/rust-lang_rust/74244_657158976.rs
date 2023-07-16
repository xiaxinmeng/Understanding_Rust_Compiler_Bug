rust
trait Allocator {
    type Buffer;
}

struct DefaultAllocator;

impl<T> Allocator for DefaultAllocator {
    type Buffer = ();
}

type A = impl Fn(<DefaultAllocator as Allocator>::Buffer);

fn foo() -> A {
    |_| ()
}
