plain
[RUSTC-TIMING] gimli test:false 5.119
error: comparison is useless due to type limits
  --> library/std/src/sys/unix/process/process_common.rs:63:33
   |
63 |             if set.is_null() || bit < 0 || bit >= (8 * size_of::<sigset_t>()) {
   |
   |
   = note: `-D unused-comparisons` implied by `-D warnings`
[RUSTC-TIMING] std test:false 4.193
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:11:53
