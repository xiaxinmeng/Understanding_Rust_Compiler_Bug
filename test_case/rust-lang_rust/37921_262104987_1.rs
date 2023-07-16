rust
fn unalign_count_ones(data: &[u8]) -> u32 {
    let mut total = 0;
    let mut iter = UnalignedIter::<u64>::from_slice(data);
    total += (&mut iter).map(|x| x.count_ones()).sum();
    total += iter.tail().map(|x| x.count_ones()).sum();
    total
}
