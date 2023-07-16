rust
[INFO] [stderr] error[E0277]: the trait bound `(): std::error::Error` is not satisfied
[INFO] [stderr]    --> src/reader/macro_src.rs:155:50
[INFO] [stderr]     |
[INFO] [stderr] 155 |             let temp = $type::from_str(&temp_str)?;
[INFO] [stderr]     |                                                  ^ the trait `std::error::Error` is not implemented for `()`
[INFO] [stderr]     | 
[INFO] [stderr]    ::: src/reader/prim_reader.rs:33:5
[INFO] [stderr]     |
[INFO] [stderr] 33  |     load_text!(f, params, String)
[INFO] [stderr]     |     ----------------------------- in this macro invocation
[INFO] [stderr]     |
[INFO] [stderr]     = note: required because of the requirements on the impl of `failure::Fail` for `()`
[INFO] [stderr]     = note: required because of the requirements on the impl of `std::convert::From<()>` for `failure::Error`
[INFO] [stderr]     = note: required by `std::convert::From::from`
