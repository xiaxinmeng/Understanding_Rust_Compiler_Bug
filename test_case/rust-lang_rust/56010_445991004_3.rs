",
[01:05:51]     "    ///",
[01:05:51]     "    /// [`rchunks`]: #method.rchunks",
[01:05:51]     "    /// [`chunks_exact`]: #method.chunks_exact"
[01:05:51] warning: `[chunks]` cannot be resolved, ignoring it...
[01:05:51]    --> src/libcore/slice/mod.rs:859:52
[01:05:51]     |
[01:05:51] 859 |     /// resulting code better than in the case of [`chunks`].
[01:05:51] 859 |     /// resulting code better than in the case of [`chunks`].
[01:05:51]     |                                                    ^^^^^^^^ cannot be resolved, ignoring
[01:05:51]     |
[01:05:51]     = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:05:51]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:51] 
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: Attributes {
[01:05:51]     doc_strings: [
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:891:5: 891:99,
[01:05:51]             src/libcore/slice/mod.rs:891:5: 891:99,
[01:05:51]             " Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:892:5: 892:22,
[01:05:51]             src/libcore/slice/mod.rs:892:5: 892:22,
[01:05:51]             " of the slice."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:893:5: 893:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:894:5: 894:95,
[01:05:51]             src/libcore/slice/mod.rs:894:5: 894:95,
[01:05:51]             " The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:895:5: 895:100,
[01:05:51]             src/libcore/slice/mod.rs:895:5: 895:100,
[01:05:51]             " length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:896:5: 896:70,
[01:05:51]             src/libcore/slice/mod.rs:896:5: 896:70,
[01:05:51]             " retrieved from the `into_remainder` function of the iterator."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:897:5: 897:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:898:5: 898:100,
[01:05:51]             src/libcore/slice/mod.rs:898:5: 898:100,
[01:05:51]             " Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:899:5: 899:66,
[01:05:51]             src/libcore/slice/mod.rs:899:5: 899:66,
[01:05:51]             " resulting code better than in the case of [`chunks_mut`]."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:900:5: 900:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:901:5: 901:96,
[01:05:51]             src/libcore/slice/mod.rs:901:5: 901:96,
[01:05:51]             " See [`rchunks_mut`] for a variant of this iterator that also returns the remainder as a"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:902:5: 902:100,
[01:05:51]             src/libcore/slice/mod.rs:902:5: 902:100,
[01:05:51]             " smaller chunk, and [`chunks_exact_mut`] for the same iterator but starting at the beginning"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:903:5: 903:35,
[01:05:51]             src/libcore/slice/mod.rs:903:5: 903:35,
[01:05:51]             " of the slice of the slice."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:904:5: 904:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:905:5: 905:17,
[01:05:51]             src/libcore/slice/mod.rs:905:5: 905:17,
[01:05:51]             " # Panics"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:906:5: 906:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:907:5: 907:37,
[01:05:51]             src/libcore/slice/mod.rs:907:5: 907:37,
[01:05:51]             " Panics if `chunk_size` is 0."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:908:5: 908:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:909:5: 909:19,
[01:05:51]             src/libcore/slice/mod.rs:909:5: 909:19,
[01:05:51]             " # Examples"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:910:5: 910:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:911:5: 911:12,
[01:05:51]             src/libcore/slice/mod.rs:911:5: 911:12,
[01:05:51]             " 