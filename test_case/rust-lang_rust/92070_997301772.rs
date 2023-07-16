plain
    Checking rand_core v0.5.1
    Checking rand_chacha v0.2.2
    Checking rand_xorshift v0.2.0
    Checking rand v0.7.3
error: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
    |
497 |             .into_iter()
    |              ^^^^^^^^^
    |
    |
    = note: `-D array-into-iter` implied by `-D warnings`
    = warning: this changes meaning in Rust 2021
    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
497 |             .iter()
    |              ~~~~
    |              ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
    |
496 |         IntoIterator::into_iter(["base", "base::test", "base::test1", "base::test2"])
    |         ++++++++++++++++++++++++                                                    ~
error: could not compile `test` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:49
