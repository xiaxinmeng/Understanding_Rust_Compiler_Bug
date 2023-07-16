
[INFO] [stdout] error: format argument must be a string literal
[INFO] [stdout]    --> src/imageops/sample.rs:904:21
[INFO] [stdout]     |
[INFO] [stdout] 904 |                     format!("alpha value: {}, {:?}", alpha, filter)
[INFO] [stdout]     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[INFO] [stdout]     |
[INFO] [stdout]     = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
[INFO] [stdout] help: you might be missing a string literal to format with
[INFO] [stdout]     |
[INFO] [stdout] 904 |                     "{}", format!("alpha value: {}, {:?}", alpha, filter)
[INFO] [stdout]     |                     +++++
