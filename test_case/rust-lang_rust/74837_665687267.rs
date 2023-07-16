
failures:

---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----
11        bb0: {
12  -         falseUnwind -> [real: bb1, cleanup: bb2]; // scope 0 at $DIR/instrument_coverage.rs:10:5: 14:6
13  +         StorageLive(_4);                 // scope 0 at $DIR/instrument_coverage.rs:9:11: 9:11
- +         _4 = const std::intrinsics::count_code_region(const 16004455475339839479_u64, const 0_u32, const 397_u32, const 465_u32) -> bb7; // scope 0 at $DIR/instrument_coverage.rs:9:11: 9:11
+ +         _4 = const std::intrinsics::count_code_region(const 16004455475339839479_u64, const 0_u32, const 425_u32, const 493_u32) -> bb7; // scope 0 at $DIR/instrument_coverage.rs:9:11: 9:11
