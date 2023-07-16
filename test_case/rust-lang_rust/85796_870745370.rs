rust
use std::sync::atomic::{AtomicUsize, Ordering};

struct Tag(usize);

#[repr(C)]
struct S(Tag, Tag, Tag);

impl Drop for Tag {
    fn drop(&mut self) {
        static A: AtomicUsize = AtomicUsize::new(0);
        match self.0 {
            0 => A.store((self as *const Tag).wrapping_add(2) as usize, Ordering::Relaxed),
            2 => assert_eq!(A.load(Ordering::Relaxed), self as *const Tag as usize),
            _ => {},
        }
    }
}

fn main() {
   S(Tag(0), Tag(1), Tag(2)).1;
}
