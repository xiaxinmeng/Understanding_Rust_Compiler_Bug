rust
trait Sweep {
    fn sweep(&self) -> usize;
}

trait SliceWrapper<T> {
    fn slice(&self) -> &[T];
}

trait SliceWrapperMut<T> {
    fn slice_mut(&mut self) -> &mut [T];
}

fn foo<B: SliceWrapper<u32> + SliceWrapperMut<u32> + Sweep>(mut buckets: B) {
    let key = 0u32;
    buckets.slice_mut()[(key as usize).wrapping_add(22).wrapping_rem(buckets.sweep())] = 22;
}

fn main() { }
