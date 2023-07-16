plain
[00:03:26]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:27]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:27]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:31]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:35] warning: function is never used: `hashmap_random_keys`
[00:03:35]   --> libstd/sys/unix/rand.rs:14:1
[00:03:35]    |
[00:03:35] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[00:03:35]    |
[00:03:35]    = note: #[warn(dead_code)] on by default
[00:03:35] 
[00:03:35] warning: function is never used: `getrandom`
[00:03:35] warning: function is never used: `getrandom`
[00:03:35]   --> libstd/sys/unix/rand.rs:36:5
[00:03:35]    |
[00:03:35] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[00:03:35] 
[00:03:35] warning: function is never used: `getrandom_fill_bytes`
[00:03:35]   --> libstd/sys/unix/rand.rs:45:5
[00:03:35]    |
[00:03:35]    |
[00:03:35] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[00:03:35] 
[00:03:35] 
[00:03:35] warning: function is never used: `is_getrandom_available`
[00:03:35]   --> libstd/sys/unix/rand.rs:67:5
[00:03:35]    |
[00:03:35] 67 |     fn is_getrandom_available() -> bool {
[00:03:35] 
[00:03:35] warning: function is never used: `fill_bytes`
[00:03:35]   --> libstd/sys/unix/rand.rs:93:5
[00:03:35]    |
[00:03:35]    |
[00:03:35] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[00:03:35] 
[00:03:42]     Finished release [optimized] target(s) in 39.13s
[00:03:42] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:03:42] travis_fold:end:stage0-std
---
[00:55:02]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:55:04] error[E0308]: mismatched types
[00:55:04]     --> liballoc/collections/hash/map.rs:3666:20
[00:55:04]      |
[00:55:04] 3666 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_USIZE) {
[00:55:04]      |                    ^^^^^^^^^^^^^^^^ expected enum `collections::CollectionAllocErr`, found enum `std::collections::CollectionAllocErr`
[00:55:04]      = note: expected type `collections::CollectionAllocErr`
[00:55:04]                 found type `std::collections::CollectionAllocErr`
[00:55:04] 
[00:55:04] error[E0308]: mismatched types
[00:55:04] error[E0308]: mismatched types
[00:55:04]     --> liballoc/collections/hash/map.rs:3670:24
[00:55:04]      |
[00:55:04] 3670 |             if let Err(CapacityOverflow) = empty_bytes.try_reserve(max_no_ovf) {
[00:55:04]      |                        ^^^^^^^^^^^^^^^^ expected enum `collections::CollectionAllocErr`, found enum `std::collections::CollectionAllocErr`
[00:55:04]      = note: expected type `collections::CollectionAllocErr`
[00:55:04]                 found type `std::collections::CollectionAllocErr`
[00:55:04] 
[00:55:04] error[E0308]: mismatched types
[00:55:04] error[E0308]: mismatched types
[00:55:04]     --> liballoc/collections/hash/map.rs:3673:24
[00:55:04]      |
[00:55:04] 3673 |             if let Err(AllocErr) = empty_bytes.try_reserve(max_no_ovf) {
[00:55:04]      |                        ^^^^^^^^ expected enum `collections::CollectionAllocErr`, found enum `std::collections::CollectionAllocErr`
[00:55:04]      = note: expected type `collections::CollectionAllocErr`
[00:55:04]                 found type `std::collections::CollectionAllocErr`
[00:55:04] 
[00:55:06] error: aborting due to 3 previous errors
