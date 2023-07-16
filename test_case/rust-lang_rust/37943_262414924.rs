
error: use of unstable library feature 'exact_size_is_empty' (see issue #35428)
   --> /checkout/src/libcollections/../libcollectionstest/slice.rs:641:40
    |
641 |             assert_eq!(xs[i..j].iter().is_empty(), xs[i..j].is_empty());
    |                                        ^^^^^^^^
    |
    = help: add #![feature(exact_size_is_empty)] to the crate attributes to enable

error: variable does not need to be mutable
   --> /checkout/src/libcollections/../libcollectionstest/slice.rs:638:9
    |
638 |     let mut xs = [1, 2, 5, 10, 11];
    |         ^^^^^^
    |
note: lint level defined here
   --> /checkout/src/libcollections/../libcollectionstest/lib.rs:11:9
    |
11  | #![deny(warnings)]
    |         ^^^^^^^^

error: aborting due to 2 previous errors

Build failed, waiting for other jobs to finish...
error: Could not compile `collections`.
