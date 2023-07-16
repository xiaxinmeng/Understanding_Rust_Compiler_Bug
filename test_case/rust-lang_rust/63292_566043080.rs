rust
/// Safe abstraction for initializing array
struct Array<T, const N: usize> {
arr: [MaybeUninit<T>; N],
position: usize
}

impl<const N: usize> Array<u8, N> {
    /// Helper for safely, efficiently reading from reader
    read<R: Read>(&mut self, reader: R) -> Result<usize, Error> {
        // ...
    }
}
