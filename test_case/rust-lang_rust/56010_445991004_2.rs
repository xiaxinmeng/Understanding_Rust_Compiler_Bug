\n    ///\n    /// [`rchunks`]: #method.rchunks\n    /// [`chunks_exact`]: #method.chunks_exact"
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: [
[01:05:51]     " Returns an iterator over `chunk_size` elements of the slice at a time, starting at the",
[01:05:51]     " beginning of the slice.",
[01:05:51]     "",
[01:05:51]     " The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the",
[01:05:51]     " slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved",
[01:05:51]     " from the `remainder` function of the iterator.",
[01:05:51]     "",
[01:05:51]     " Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the",
[01:05:51]     " resulting code better than in the case of [`chunks`]."
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: [
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: [
[01:05:51]     "/// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the",
[01:05:51]     "    /// beginning of the slice.",
[01:05:51]     "    ///",
[01:05:51]     "    /// The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the",
[01:05:51]     "    /// slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved",
[01:05:51]     "    /// from the `remainder` function of the iterator.",
[01:05:51]     "    ///",
[01:05:51]     "    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the",
[01:05:51]     "    /// resulting code better than in the case of [`chunks`].",
[01:05:51]     "    ///",
[01:05:51]     "    /// See [`rchunks`] for a variant of this iterator that also returns the remainder as a smaller",
[01:05:51]     "    /// chunk, and [`chunks_exact`] for the same iterator but starting at the beginning of the",
[01:05:51]     "    /// slice of the slice.",
[01:05:51]     "    ///",
[01:05:51]     "    /// # Panics",
[01:05:51]     "    ///",
[01:05:51]     "    /// Panics if `chunk_size` is 0.",
[01:05:51]     "    ///",
[01:05:51]     "    /// # Examples",
[01:05:51]     "    ///",
[01:05:51]     "    /// 