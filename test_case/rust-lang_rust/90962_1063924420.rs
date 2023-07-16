rust
#![feature(const_align_offset)]
#![feature(const_mut_refs)]

#[repr(align(4096))]
struct PageAligned<T>(T);
static mut MEMORY: PageAligned<[u8; 1]> = PageAligned([0; 1]);

struct Foo;
impl Foo {
    const fn new(mem: &[u8]) -> Self {
        assert!(mem.as_ptr().align_offset(4096) == 0, "memory must be page aligned!");
        Self
    }
}
static FOO: Foo = unsafe {Foo::new(&mut MEMORY.0)};


fn main() {}
