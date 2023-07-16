
// Manually inlining this prevents LLVM turning it into a memcpy
unsafe fn clone_from_existing<T: Clone>(dst: &mut Vec<T>, src: &[T]) {
    for i in 0..dst.len() {
        dst.get_unchecked_mut(i).clone_from(src.get_unchecked(i));
    }
}

self.truncate(other.len());

unsafe { clone_from_existing(self, &other); }

let len = self.len();
if len != other.len() {
    self.push_all(&other[len..]);
}
