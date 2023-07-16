rust
struct PermutedDrop<A: Array> {
    data: [ManuallyDrop<A::Item>; A::LENGTH],
    order: [usize; A::LENGTH]
}

impl<A: Array> Drop for PermutedDrop<A> {
    fn drop(&mut self) {
        for idx in self.order {
            unsafe {
                ManuallyDrop::drop(&mut self.data[idx]);
            }
        }
    }
}
