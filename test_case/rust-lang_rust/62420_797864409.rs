rust
#[derive(Clone, Copy)]
pub struct TaggedPtr(*const u8);

impl TaggedPtr {
    /// # Safety
    ///
    /// * `ptr` must be properly aligned (i.e., have an alignment of at least 2).
    /// * `ptr` must be “dereferencable” in the sense defined by [`std::ptr`](std::ptr#safety).
    pub unsafe fn new(ptr: *const u16, tag: bool) -> Self {
        Self(unsafe { (ptr as *const u8).add(tag as usize) })
    }

    /// Returns the stored pointer and tag.
    pub fn get(self) -> (*const u16, bool) {
        let offset = self.0.align_offset(2);
        assert!(offset != usize::MAX); // Ideally, this would be guaranteed.

        let tag = offset & 1;
        (unsafe { self.0.sub(tag) } as *const u16, tag != 0)
    }
}

fn main() {
    let i1 = 0_u16;
    let i2 = 1_u16;

    let p1 = &i1 as *const u16;
    let p2 = &i2 as *const u16;

    let t1 = unsafe { TaggedPtr::new(p1, true) };
    let t2 = unsafe { TaggedPtr::new(p2, false) };

    assert_eq!(t1.get(), (p1, true));
    assert_eq!(t2.get(), (p2, false));
}
