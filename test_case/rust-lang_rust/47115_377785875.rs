rust
// Unsigned integer division rounds toward zero / truncates
let (chunkable, extra) = slice.split_at(slice.len() / chunk_size * chunk_size)
