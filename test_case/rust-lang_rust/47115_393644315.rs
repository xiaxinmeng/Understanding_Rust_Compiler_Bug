rust
struct ExactChunks<T> {
    chunk_size: usize,
    chunk_start: *const T,
    rem_start: *const T,
    rem_end: *const T, // can be replaced by the length of the remaining part
}
