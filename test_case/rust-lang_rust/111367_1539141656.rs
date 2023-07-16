plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the trait bound `Range<usize>: ~const IntoIterator` is not satisfied
   --> library/core/src/str/validations.rs:224:26
    |
224 |                 for _ in 0..offset {
    |                          ^^^^^^^^^ the trait `~const IntoIterator` is not implemented for `Range<usize>`
    |
note: the trait `IntoIterator` is implemented for `Range<usize>`, but that implementation is not `const`
   --> library/core/src/str/validations.rs:224:26
    |
224 |                 for _ in 0..offset {

error[E0015]: cannot convert `Range<usize>` into an iterator in constant functions
   --> library/core/src/str/validations.rs:224:26
    |
    |
224 |                 for _ in 0..offset {
    |
    |
note: impl defined here, but it is not `const`
    |
    |
266 | impl<I: Iterator> IntoIterator for I {
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0277]: the trait bound `Range<usize>: ~const Iterator` is not satisfied
   --> library/core/src/str/validations.rs:224:26
    |
224 |                 for _ in 0..offset {
    |                          ^^^^^^^^^ `Range<usize>` is not an iterator
    |
    = help: the trait `~const Iterator` is not implemented for `Range<usize>`
note: the trait `Iterator` is implemented for `Range<usize>`, but that implementation is not `const`
   --> library/core/src/str/validations.rs:224:26
    |
224 |                 for _ in 0..offset {


error[E0015]: cannot call non-const fn `<Range<usize> as Iterator>::next` in constant functions
   --> library/core/src/str/validations.rs:224:26
    |
224 |                 for _ in 0..offset {
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

Some errors have detailed explanations: E0015, E0277.
