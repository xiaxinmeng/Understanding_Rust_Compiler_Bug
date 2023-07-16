
[INFO] [stdout] error: internal compiler error: failed to process buffered lint here (dummy = false)
[INFO] [stdout]    --> atl-checker/src/emit_count.rs:5:49
[INFO] [stdout]     |
[INFO] [stdout] 5   |         eprintln!(concat!("[stats] ", $($msg)*));
[INFO] [stdout]     |                                                 ^
[INFO] [stdout]     |
[INFO] [stdout]    ::: atl-checker/src/algorithms/certain_zero/mod.rs:344:21
[INFO] [stdout]     |
[INFO] [stdout] 344 |                     emit_count!("worker received_dirty_token")
[INFO] [stdout]     |                     ------------------------------------------ in this macro invocation
[INFO] [stdout]     |
[INFO] [stdout]     = note: delayed at /rustc/f9a839ea0c4c4885a5366d877a75ad3525bbab5e/compiler/rustc_lint/src/early.rs:402:18
[INFO] [stdout]     = note: this error: internal compiler error originates in the macro `emit_count` (in Nightly builds, run with -Z macro-backtrace for more info)
