plain
   Compiling core v0.0.0 (/checkout/library/core)
error: empty for iterator
   --> library/core/tests/iter/range.rs:464:5
    |
464 |     for _ in (10..0).rev() {
    |
    |
    = note: `-D empty-for-iterator` implied by `-D warnings`
error: empty for iterator
   --> library/core/tests/iter/range.rs:469:5
    |
    |
469 |     for _ in (10..0).rev() {

error: could not compile `core` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:19:24
