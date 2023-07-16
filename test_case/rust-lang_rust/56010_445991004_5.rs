\n    ///\n    /// [`rchunks_mut`]: #method.rchunks_mut\n    /// [`chunks_exact_mut`]: #method.chunks_exact_mut"
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: [
[01:05:51]     " Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end",
[01:05:51]     " of the slice.",
[01:05:51]     "",
[01:05:51]     " The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the",
[01:05:51]     " length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be",
[01:05:51]     " retrieved from the `into_remainder` function of the iterator.",
[01:05:51]     "",
[01:05:51]     " Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the",
[01:05:51]     " resulting code better than in the case of [`chunks_mut`]."
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: [
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: [
[01:05:51]     "/// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end",
[01:05:51]     "    /// of the slice.",
[01:05:51]     "    ///",
[01:05:51]     "    /// The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the",
[01:05:51]     "    /// length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be",
[01:05:51]     "    /// retrieved from the `into_remainder` function of the iterator.",
[01:05:51]     "    ///",
[01:05:51]     "    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the",
[01:05:51]     "    /// resulting code better than in the case of [`chunks_mut`].",
[01:05:51]     "    ///",
[01:05:51]     "    /// See [`rchunks_mut`] for a variant of this iterator that also returns the remainder as a",
[01:05:51]     "    /// smaller chunk, and [`chunks_exact_mut`] for the same iterator but starting at the beginning",
[01:05:51]     "    /// of the slice of the slice.",
[01:05:51]     "    ///",
[01:05:51]     "    /// # Panics",
[01:05:51]     "    ///",
[01:05:51]     "    /// Panics if `chunk_size` is 0.",
[01:05:51]     "    ///",
[01:05:51]     "    /// # Examples",
[01:05:51]     "    ///",
[01:05:51]     "    /// 