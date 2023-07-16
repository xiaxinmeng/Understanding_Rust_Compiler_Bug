rust
[INFO] [stderr] error[E0658]: use of unstable library feature 'matches_macro'
[INFO] [stderr]    --> src/request.rs:152:21
[INFO] [stderr]     |
[INFO] [stderr] 152 |             assert!(matches!(&new_request, Request::Delete(ref _key)));
[INFO] [stderr]     |                     ^^^^^^^
[INFO] [stderr] ...
[INFO] [stderr] 162 |                 assert_delete!(b"name");
[INFO] [stderr]     |                 ------------------------ in this macro invocation
[INFO] [stderr]     |
[INFO] [stderr]     = note: for more information, see https://github.com/rust-lang/rust/issues/65721
