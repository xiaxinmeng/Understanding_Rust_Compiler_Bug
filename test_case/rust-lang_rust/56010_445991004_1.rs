"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:879:5: 879:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:880:5: 880:37,
[01:05:51]             src/libcore/slice/mod.rs:880:5: 880:37,
[01:05:51]             " [`rchunks`]: #method.rchunks"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:881:5: 881:47,
[01:05:51]             src/libcore/slice/mod.rs:881:5: 881:47,
[01:05:51]             " [`chunks_exact`]: #method.chunks_exact"
[01:05:51]     ],
[01:05:51]     other_attrs: [
[01:05:51]         Attribute {
[01:05:51]             id: AttrId(
[01:05:51]             id: AttrId(
[01:05:51]                 22443
[01:05:51]             ),
[01:05:51]             style: Outer,
[01:05:51]             path: path(stable),
[01:05:51]             tokens: TokenStream {
[01:05:51]                 kind: Tree(
[01:05:51]                     Delimited(
[01:05:51]                         DelimSpan {
[01:05:51]                             open: src/libcore/slice/mod.rs:882:13: 882:14,
[01:05:51]                             close: src/libcore/slice/mod.rs:882:51: 882:52
[01:05:51]                         Paren,
[01:05:51]                         ThinTokenStream(
[01:05:51]                             Some(
[01:05:51]                                 [
---
[01:05:51]         Attribute {
[01:05:51]             id: AttrId(
[01:05:51]                 22444
[01:05:51]             ),
[01:05:51]             style: Outer,
[01:05:51]             path: path(inline),
[01:05:51]             tokens: TokenStream {
[01:05:51]                 kind: Empty
[01:05:51]             is_sugared_doc: false,
[01:05:51]             span: src/libcore/slice/mod.rs:883:5: 883:14
[01:05:51]         }
[01:05:51]     ],
[01:05:51]     ],
[01:05:51]     cfg: None,
[01:05:51]     span: Some(
[01:05:51]         src/libcore/slice/mod.rs:851:5: 851:95
[01:05:51]     ),
[01:05:51]     links: [],
[01:05:51]     inner_docs: false
[01:05:51] }
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: src/libcore/slice/mod.rs:851:5: 881:47
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: "/// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the\n    /// beginning of the slice.\n    ///\n    /// The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the\n    /// slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved\n    /// from the `remainder` function of the iterator.\n    ///\n    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the\n    /// resulting code better than in the case of [`chunks`].\n    ///\n    /// See [`rchunks`] for a variant of this iterator that also returns the remainder as a smaller\n    /// chunk, and [`chunks_exact`] for the same iterator but starting at the beginning of the\n    /// slice of the slice.\n    ///\n    /// # Panics\n    ///\n    /// Panics if `chunk_size` is 0.\n    ///\n    /// # Examples\n    ///\n    /// 