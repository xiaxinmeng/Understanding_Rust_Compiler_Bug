
error: an inner attribute is not permitted in this context
    --> library/core/tests/iter.rs:3226:1
     |
3226 | #![feature(iterator_fold_mut)]
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
