rust
pub trait PushUnchecked<T> {
    unsafe fn push_unchecked(&mut self, value: T);
}
impl<T> PushUnchecked<T> for Vec<T> {
    #[inline]
    unsafe fn push_unchecked(&mut self, value: T) {
        debug_assert!(self.len() < self.capacity());
        if self.len() == self.capacity() {
            core::hint::unreachable_unchecked();
        }
        self.push(value);
    }
}

fn cond(i: usize) -> bool {
    i % 2 == 0
}
struct SomeData(usize);
fn main() {
    const N: usize = 1_000_000;
    let mut data = Vec::with_capacity(N);

    for i in 0..N {
        if cond(i) {
            unsafe { data.push_unchecked(SomeData(i)) };
        }
    }
}
