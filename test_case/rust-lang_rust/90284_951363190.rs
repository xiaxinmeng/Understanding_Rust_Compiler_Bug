plain
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.11.0
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0425]: cannot find value `Aligner` in this scope
   |
   |
22 |         CacheAligned(t, Aligner)

error[E0061]: this struct takes 1 argument but 2 arguments were supplied
  --> library/std/src/sync/mpsc/cache_aligned.rs:22:9
   |
   |
22 |         CacheAligned(t, Aligner)
   |         ^^^^^^^^^^^^ -  ------- supplied 2 arguments
   |         expected 1 argument
   |
note: tuple struct defined here
  --> library/std/src/sync/mpsc/cache_aligned.rs:5:19
  --> library/std/src/sync/mpsc/cache_aligned.rs:5:19
   |
5  | pub(super) struct CacheAligned<T>(pub T);

Some errors have detailed explanations: E0061, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `std` due to 2 previous errors
