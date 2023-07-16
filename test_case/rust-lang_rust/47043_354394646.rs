rust
#[unstable(feature = "asdfasdf", issue = "12345678")]
#[derive(Debug)]
pub struct FooAlloc(usize);

#[unstable(feature = "asdfasdf", issue = "12345678")]
unsafe impl ::allocator::Alloc for FooAlloc {
    type Err = !;

    unsafe fn alloc(&mut self, layout: ::allocator::Layout) -> Result<*mut u8, Self::Err> {
        ::abort_adapter::AbortAdapter(::heap::Heap).alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: ::allocator::Layout) {
        ::abort_adapter::AbortAdapter(::heap::Heap).dealloc(ptr, layout)
    }
}

#[unstable(feature = "asdfasdf", issue = "12345678")]
pub fn foo(x: Box<String, FooAlloc>, y: bool) { if y { let _x = *x; } }
