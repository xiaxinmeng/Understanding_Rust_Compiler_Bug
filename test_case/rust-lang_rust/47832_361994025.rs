
[01:02:28] ---- [compile-fail] compile-fail/integral-indexing.rs stdout ----
[01:02:28] 	
[01:02:28] error: /checkout/src/test/compile-fail/integral-indexing.rs:16: unexpected error: '16:5: 16:11: the trait bound `u8: std::slice::SliceIndex<[isize]>` is not satisfied [E0277]'
[01:02:28] 
[01:02:28] error: /checkout/src/test/compile-fail/integral-indexing.rs:17: unexpected error: '17:5: 17:11: the trait bound `i8: std::slice::SliceIndex<[isize]>` is not satisfied [E0277]'
[01:02:28] 
[01:02:28] error: /checkout/src/test/compile-fail/integral-indexing.rs:18: unexpected error: '18:5: 18:12: the trait bound `u32: std::slice::SliceIndex<[isize]>` is not satisfied [E0277]'
[01:02:28] 
[01:02:28] error: /checkout/src/test/compile-fail/integral-indexing.rs:19: unexpected error: '19:5: 19:12: the trait bound `i32: std::slice::SliceIndex<[isize]>` is not satisfied [E0277]'
[01:02:28] 
[01:02:28] error: /checkout/src/test/compile-fail/integral-indexing.rs:16: expected error not found: : std::ops::Index<u8>` is not satisfied
[01:02:28] 
[01:02:28] error: /checkout/src/test/compile-fail/integral-indexing.rs:17: expected error not found: : std::ops::Index<i8>` is not satisfied
[01:02:28] 
[01:02:28] error: /checkout/src/test/compile-fail/integral-indexing.rs:18: expected error not found: : std::ops::Index<u32>` is not satisfied
[01:02:28] 
[01:02:28] error: /checkout/src/test/compile-fail/integral-indexing.rs:19: expected error not found: : std::ops::Index<i32>` is not satisfied
[01:02:28] 
[01:02:28] error: 4 unexpected errors found, 4 expected errors not found
