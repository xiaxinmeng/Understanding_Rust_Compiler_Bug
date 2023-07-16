plain
   Compiling rustc-demangle v0.1.21
error: unnecessary parentheses around assigned value
   --> library/alloc/src/raw_vec.rs:404:24
    |
404 |         let bin_size = (usize::MAX >> (alloc_size - 1).leading_zeros()); // + 1 skipped to prevent overflow
    |                        ^                                              ^
    |
    = note: `-D unused-parens` implied by `-D warnings`
help: remove these parentheses
    |
404 -         let bin_size = (usize::MAX >> (alloc_size - 1).leading_zeros()); // + 1 skipped to prevent overflow
404 +         let bin_size = usize::MAX >> (alloc_size - 1).leading_zeros(); // + 1 skipped to prevent overflow

error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:15
