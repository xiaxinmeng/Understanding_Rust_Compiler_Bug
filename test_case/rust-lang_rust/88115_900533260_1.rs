
[INFO] [stdout] warning: panic message is not a string literal
[INFO] [stdout]    --> src/imageops/sample.rs:904:21
[INFO] [stdout]     |
[INFO] [stdout] 904 |                     format!("alpha value: {}, {:?}", alpha, filter)
[INFO] [stdout]     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[INFO] [stdout]     |
[INFO] [stdout]     = note: this usage of assert!() is deprecated; it will be a hard error in Rust 2021
[INFO] [stdout]     = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/panic-macro-consistency.html>
[INFO] [stdout]     = note: the assert!() macro supports formatting, so there's no need for the format!() macro here
[INFO] [stdout] help: remove the `format!(..)` macro call
[INFO] [stdout]     |
[INFO] [stdout] 904 -                     format!("alpha value: {}, {:?}", alpha, filter)
[INFO] [stdout] 904 +                     "alpha value: {}, {:?}", alpha, filter
[INFO] [stdout]     | 
