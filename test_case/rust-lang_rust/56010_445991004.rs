plain
travis_time:end:14df6a90:start=1544473859286344681,finish=1544473916768307887,duration=57481963206
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-i686-linux
---
[01:05:31]     Checking compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[01:05:33]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[01:05:36]     Finished release [optimized] target(s) in 33.51s
[01:05:37]  Documenting core v0.0.0 (/checkout/src/libcore)
[01:05:51] ERROR 2018-12-10T21:37:56Z: rustdoc::passes::collect_intra_doc_links: Attributes {
[01:05:51]     doc_strings: [
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:851:5: 851:95,
[01:05:51]             src/libcore/slice/mod.rs:851:5: 851:95,
[01:05:51]             " Returns an iterator over `chunk_size` elements of the slice at a time, starting at the"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:852:5: 852:32,
[01:05:51]             src/libcore/slice/mod.rs:852:5: 852:32,
[01:05:51]             " beginning of the slice."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:853:5: 853:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:854:5: 854:100,
[01:05:51]             src/libcore/slice/mod.rs:854:5: 854:100,
[01:05:51]             " The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:855:5: 855:96,
[01:05:51]             src/libcore/slice/mod.rs:855:5: 855:96,
[01:05:51]             " slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:856:5: 856:55,
[01:05:51]             src/libcore/slice/mod.rs:856:5: 856:55,
[01:05:51]             " from the `remainder` function of the iterator."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:857:5: 857:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:858:5: 858:100,
[01:05:51]             src/libcore/slice/mod.rs:858:5: 858:100,
[01:05:51]             " Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:859:5: 859:62,
[01:05:51]             src/libcore/slice/mod.rs:859:5: 859:62,
[01:05:51]             " resulting code better than in the case of [`chunks`]."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:860:5: 860:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:861:5: 861:100,
[01:05:51]             src/libcore/slice/mod.rs:861:5: 861:100,
[01:05:51]             " See [`rchunks`] for a variant of this iterator that also returns the remainder as a smaller"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:862:5: 862:95,
[01:05:51]             src/libcore/slice/mod.rs:862:5: 862:95,
[01:05:51]             " chunk, and [`chunks_exact`] for the same iterator but starting at the beginning of the"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:863:5: 863:28,
[01:05:51]             src/libcore/slice/mod.rs:863:5: 863:28,
[01:05:51]             " slice of the slice."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:864:5: 864:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:865:5: 865:17,
[01:05:51]             src/libcore/slice/mod.rs:865:5: 865:17,
[01:05:51]             " # Panics"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:866:5: 866:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:867:5: 867:37,
[01:05:51]             src/libcore/slice/mod.rs:867:5: 867:37,
[01:05:51]             " Panics if `chunk_size` is 0."
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:868:5: 868:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:869:5: 869:19,
[01:05:51]             src/libcore/slice/mod.rs:869:5: 869:19,
[01:05:51]             " # Examples"
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:870:5: 870:8,
[01:05:51]             ""
[01:05:51]         ),
[01:05:51]         ),
[01:05:51]         SugaredDoc(
[01:05:51]             src/libcore/slice/mod.rs:871:5: 871:12,
[01:05:51]             src/libcore/slice/mod.rs:871:5: 871:12,
[01:05:51]             " 