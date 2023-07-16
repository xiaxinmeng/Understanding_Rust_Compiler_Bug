
May 25 11:49:29.066 INFO kablam! error[E0283]: type annotations required: cannot resolve `std::borrow::Cow<'_, str>: std::convert::From<&_>`
May 25 11:49:29.066 INFO kablam!    --> src/main.rs:541:17
May 25 11:49:29.066 INFO kablam!     |
May 25 11:49:29.066 INFO kablam! 541 |                 Cow::from(options.pattern.as_ref())
May 25 11:49:29.066 INFO kablam!     |                 ^^^^^^^^^
May 25 11:49:29.066 INFO kablam!     |
May 25 11:49:29.066 INFO kablam!     = note: required by `std::convert::From::from`
