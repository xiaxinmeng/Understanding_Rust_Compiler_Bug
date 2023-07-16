plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: implementation has missing stability attribute
   --> library/core/src/iter/adapters/chain.rs:285:1
    |
285 | / impl<A, B> ExactSizeIterator for Chain<A, B>
286 | | where
287 | |     A: ExactSizeIterator,
288 | |     B: ExactSizeIterator,
289 | | {
    | |_^


error[E0271]: type mismatch resolving `<B as Iterator>::Item == <A as Iterator>::Item`
    |
    |
285 | impl<A, B> ExactSizeIterator for Chain<A, B>
    |      -  -  ^^^^^^^^^^^^^^^^^ expected type parameter `A`, found type parameter `B`
    |      |  found type parameter
    |      expected type parameter
    |
    = note: expected associated type `<A as Iterator>::Item`
    = note: expected associated type `<A as Iterator>::Item`
               found associated type `<B as Iterator>::Item`
    = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
    = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
note: required because of the requirements on the impl of `Iterator` for `Chain<A, B>`
    |
    |
41  | impl<A, B> Iterator for Chain<A, B>
note: required by a bound in `ExactSizeIterator`
   --> library/core/src/iter/traits/exact_size.rs:76:30
    |
76  | pub trait ExactSizeIterator: Iterator {
