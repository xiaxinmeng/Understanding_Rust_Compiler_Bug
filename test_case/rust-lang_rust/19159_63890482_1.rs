
enum RingBufSlice<'a, T> {
    Contiguous(&'a [T]),
    Split((&'a [T], &'a [T])),
}
