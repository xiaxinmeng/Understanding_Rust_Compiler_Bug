"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:923:5: 923:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:924:5: 924:45,
[01:05:51]             src/libcore/slice/mod.rs:924:5: 924:45,
[01:05:51]             " [`rchunks_mut`]: #method.rchunks_mut"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:925:5: 925:55,
[01:05:51]             src/libcore/slice/mod.rs:925:5: 925:55,
[01:05:51]             " [`chunks_exact_mut`]: #method.chunks_exact_mut"
[01:05:51]     ],
[01:05:51]     other_attrs: [
[01:05:51]         Attribute {
[01:05:51]             id: AttrId(
[01:05:51]             id: AttrId(
[01:05:51]                 22480
[01:05:51]             ),
[01:05:51]             style: Outer,
[01:05:51]             path: path(stable),
[01:05:51]             tokens: TokenStream {
[01:05:51]                 kind: Tree(
[01:05:51]                     Delimited(
[01:05:51]                         DelimSpan {
[01:05:51]                             open: src/libcore/slice/mod.rs:926:13: 926:14,
[01:05:51]                             close: src/libcore/slice/mod.rs:926:51: 926:52
[01:05:51]                         Paren,
[01:05:51]                         ThinTokenStream(
[01:05:51]                             Some(
[01:05:51]                                 [
---
[01:05:51]         Attribute {
[01:05:51]             id: AttrId(
[01:05:51]                 22481
[01:05:51]             ),
[01:05:51]             style: Outer,
[01:05:51]             path: path(inline),
[01:05:51]             tokens: TokenStream {
[01:05:51]                 kind: Empty
[01:05:51]             is_sugared_doc: false,
[01:05:51]             span: src/libcore/slice/mod.rs:927:5: 927:14
[01:05:51]         }
[01:05:51]     ],
[01:05:51]     ],
[01:05:51]     cfg: None,
[01:05:51]     span: Some(
[01:05:51]         src/libcore/slice/mod.rs:891:5: 891:99
[01:05:51]     ),
[01:05:51]     links: [],
[01:05:51]     inner_docs: false
[01:05:51] }
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: src/libcore/slice/mod.rs:891:5: 925:55
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: "/// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end\n    /// of the slice.\n    ///\n    /// The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the\n    /// length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be\n    /// retrieved from the `into_remainder` function of the iterator.\n    ///\n    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the\n    /// resulting code better than in the case of [`chunks_mut`].\n    ///\n    /// See [`rchunks_mut`] for a variant of this iterator that also returns the remainder as a\n    /// smaller chunk, and [`chunks_exact_mut`] for the same iterator but starting at the beginning\n    /// of the slice of the slice.\n    ///\n    /// # Panics\n    ///\n    /// Panics if `chunk_size` is 0.\n    ///\n    /// # Examples\n    ///\n    /// 