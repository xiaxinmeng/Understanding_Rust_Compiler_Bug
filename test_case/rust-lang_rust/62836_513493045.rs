rust
fn floor_log2(x: usize) -> usize {
    const BITS_PER_BYTE: usize = 8;

    BITS_PER_BYTE * std::mem::size_of<usize>() - (x.leading_zeros() as usize) - 1
}
